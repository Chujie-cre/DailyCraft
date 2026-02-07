import { ISpriteConfig } from "../types/ISpriteConfig";

export class ConfigManager {
    // Config for sprite sheet that's going to be loaded
    private spriteConfig: ISpriteConfig[] = [];
    // Phaser loader plugin
    private load: Phaser.Loader.LoaderPlugin | undefined;
    // Phaser texture manager
    private textures: Phaser.Textures.TextureManager | undefined;
    // Phaser anims manager
    private anims: Phaser.Animations.AnimationManager | undefined;
    // List of registered sprite name to avoid loading duplicate sprite
    private registeredName: Map<string, boolean> = new Map();

    // fps for sprite animation
    public readonly FRAME_RATE: number;
    // repeat for sprite animation after it's done, -1 means repeat forever
    private readonly REPEAT: number = -1;

    constructor({ FRAME_RATE }: { FRAME_RATE: number }) {
        this.FRAME_RATE = FRAME_RATE;
    }

    public loadAllSpriteSheet(): void {
        try {
            if (!this.spriteConfig) return;
            this.spriteConfig.forEach((sprite) => {
                this.loadSpriteSheet(sprite);
            });
        } catch (error) {
            console.log("Error in ConfigManager loadAllSpriteSheet()", error);
        }
    }

    public registerSpriteStateAnimation(sprite: ISpriteConfig): void {
        if (!this.anims || !this.load) return;

        // avoid showing broken sprite
        if (!this.validatePetSprite(sprite)) return;

        // in case sprite hasn't loaded yet, we load it
        if (this.textures && !this.textures.exists(sprite.name)) {
            this.loadSpriteSheet(sprite);
            this.load.start();

            this.load.once("complete", () => {
                this.registerSpriteStateAnimation(sprite);
            });
            return;
        }

        // convert sprite states to lowercase
        for (const state in sprite.states) {
            if (state.toLowerCase() !== state) {
                sprite.states[state.toLowerCase()] = sprite.states[state];
                delete sprite.states[state];
            }
        }

        // register state animations for the sprite
        for (const animationConfig of this.getAnimationConfigPerSprite(sprite)) {
            if (!this.anims.exists(animationConfig.key)) {
                this.anims.create(animationConfig);
            }
        }
    }

    public setConfigManager({
        load,
        textures,
        anims,
    }: {
        load: Phaser.Loader.LoaderPlugin;
        textures: Phaser.Textures.TextureManager;
        anims: Phaser.Animations.AnimationManager;
    }): void {
        this.load = load;
        this.textures = textures;
        this.anims = anims;
    }

    public setSpriteConfig(spriteConfig: ISpriteConfig[]): void {
        this.spriteConfig = spriteConfig;
    }

    public getSpriteConfig(): ISpriteConfig[] {
        return this.spriteConfig;
    }

    private loadSpriteSheet(sprite: ISpriteConfig): void {
        if (!this.load) return;

        if (this.checkDuplicateName(sprite.name)) return;
        if (!this.validatePetSprite(sprite)) return;

        this.load.spritesheet({
            key: sprite.name,
            url: sprite.imageSrc,
            frameConfig: this.getFrameSize(sprite),
        });
    }

    private getAnimationConfigPerSprite(sprite: ISpriteConfig): {
        key: string;
        frames: Phaser.Types.Animations.AnimationFrame[];
        frameRate: number;
        repeat: number;
    }[] {
        if (!sprite.states || !this.anims) return [];

        const animationConfig = [];
        const HighestFrameMax = this.getHighestFrameMax(sprite);
        for (const state in sprite.states) {
            const start =
                sprite.states[state].start !== undefined
                    ? sprite.states[state].start! - 1
                    : (sprite.states[state].spriteLine! - 1) * HighestFrameMax;
            const end =
                sprite.states[state].end !== undefined
                    ? sprite.states[state].end! - 1
                    : start + sprite.states[state].frameMax! - 1;

            animationConfig.push({
                key: `${state}-${sprite.name}`,
                frames: this.anims.generateFrameNumbers(sprite.name, {
                    start: start,
                    end: end,
                    first: start,
                }),
                frameRate: this.FRAME_RATE,
                repeat: this.REPEAT,
            });
        }
        return animationConfig;
    }

    public getStateName(
        state: string,
        pet: Phaser.Types.Physics.Arcade.SpriteWithDynamicBody
    ): string {
        return `${state}-${pet.texture.key}`;
    }

    private getHighestFrameMax(sprite: ISpriteConfig): number {
        if (sprite.highestFrameMax) return sprite.highestFrameMax;

        let highestFrameMax = 0;
        for (const state in sprite.states) {
            if (!sprite.states[state].frameMax!) return 0;
            highestFrameMax = Math.max(
                highestFrameMax,
                sprite.states[state].frameMax!
            );
        }
        return highestFrameMax;
    }

    public getFrameSize(sprite: ISpriteConfig): {
        frameWidth: number;
        frameHeight: number;
    } {
        if (sprite.frameSize) {
            return {
                frameWidth: sprite.frameSize,
                frameHeight: sprite.frameSize,
            };
        }

        const frameWidth = sprite.width! / sprite.highestFrameMax!;
        const frameHeight = sprite.height! / sprite.totalSpriteLine!;
        return { frameWidth, frameHeight };
    }

    private checkDuplicateName(name: string): boolean {
        if (this.registeredName.has(name)) return true;
        this.registeredName.set(name, true);
        return false;
    }

    private validatePetSprite(sprite: ISpriteConfig): boolean {
        if (!sprite.name || !sprite.imageSrc || !sprite.states) return false;

        if (
            !sprite.frameSize &&
            (!sprite.width ||
                !sprite.height ||
                !sprite.highestFrameMax ||
                !sprite.totalSpriteLine)
        ) {
            return false;
        }

        for (const state in sprite.states) {
            if (
                (!sprite.states[state].spriteLine ||
                    !sprite.states[state].frameMax) &&
                (!sprite.states[state].start || !sprite.states[state].end)
            ) {
                return false;
            }
        }

        return true;
    }
}
