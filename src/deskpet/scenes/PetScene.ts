import Phaser from "phaser";
import { ISpriteConfig } from "../types/ISpriteConfig";
import {
    Direction,
    IWorldBounding,
    ISwitchStateOptions,
    Ease,
} from "../types/IPet";
import { ConfigManager, InputManager } from "../managers";

interface Pet extends Phaser.Types.Physics.Arcade.SpriteWithDynamicBody {
    direction?: Direction;
    availableStates: string[];
    canPlayRandomState: boolean;
    canRandomFlip: boolean;
    id: string;
}

export default class PetScene extends Phaser.Scene {
    private pets: Pet[] = [];
    private isFlipped: boolean = false;
    private frameCount: number = 0;
    private petClimbAndCrawlIndex: number[] = [];

    private configManager: ConfigManager;
    private inputManager: InputManager;

    // app settings
    private allowPetInteraction: boolean = true;
    private allowPetAboveTaskbar: boolean = false;
    private allowPetClimbing: boolean = true;

    private readonly FORBIDDEN_RAND_STATE: string[] = [
        "fall",
        "climb",
        "drag",
        "crawl",
        "bounce",
        "jump",
    ];
    private readonly FRAME_RATE: number = 9;
    private readonly UPDATE_DELAY: number = 1000 / this.FRAME_RATE;
    private readonly PET_MOVE_VELOCITY: number = this.FRAME_RATE * 6;
    private readonly PET_MOVE_ACCELERATION: number = this.PET_MOVE_VELOCITY * 2;
    private readonly TWEEN_ACCELERATION: number = this.FRAME_RATE * 1.1;
    private readonly RAND_STATE_DELAY: number = 6000;
    private readonly FLIP_DELAY: number = 10000;

    constructor() {
        super({ key: "PetScene" });

        this.configManager = new ConfigManager({
            FRAME_RATE: this.FRAME_RATE,
        });
        this.inputManager = new InputManager();
    }

    preload(): void {
        this.configManager.setConfigManager({
            load: this.load,
            textures: this.textures,
            anims: this.anims,
        });

        this.inputManager.setInputManager({ input: this.input });
        const spriteConfig = this.game.registry.get("spriteConfig");
        this.configManager.setSpriteConfig(spriteConfig);
        this.configManager.loadAllSpriteSheet();
    }

    create(): void {
        this.inputManager.turnOnIgnoreCursorEvents();
        this.physics.world.setBoundsCollision(true, true, true, true);
        this.updatePetAboveTaskbar();

        let i = 0;
        for (const sprite of this.configManager.getSpriteConfig()) {
            this.addPet(sprite, i);
            i++;
        }

        // register drag event
        this.input.on(
            "drag",
            (_pointer: any, pet: Pet, dragX: number, dragY: number) => {
                pet.x = dragX;
                pet.y = dragY;

                if (
                    pet.anims &&
                    pet.anims.getName() !==
                        this.configManager.getStateName("drag", pet)
                ) {
                    this.switchState(pet, "drag");
                }

                // disable world bounds when dragging
                if (pet.body!.enable) pet.body!.enable = false;

                if (pet.x > pet.input!.dragStartX) {
                    if (this.isFlipped) {
                        this.toggleFlipX(pet);
                        this.isFlipped = false;
                    }
                } else {
                    if (!this.isFlipped) {
                        this.toggleFlipX(pet);
                        this.isFlipped = true;
                    }
                }
            }
        );

        this.input.on("dragend", (pointer: any, pet: Pet) => {
            const bounds = this.physics.world.bounds;
            const targetX = Phaser.Math.Clamp(
                pet.x + pointer.velocity.x * this.TWEEN_ACCELERATION,
                0,
                bounds.width
            );
            const targetY = Phaser.Math.Clamp(
                pet.y + pointer.velocity.y * this.TWEEN_ACCELERATION,
                0,
                bounds.height
            );

            this.tweens.add({
                targets: pet,
                x: targetX,
                y: targetY,
                duration: 600,
                ease: Ease.QuartEaseOut,
                onComplete: () => {
                    if (!pet.body!.enable) {
                        pet.body!.enable = true;

                        setTimeout(() => {
                            switch (pet.anims.getName()) {
                                case this.configManager.getStateName("climb", pet):
                                    this.updateDirection(pet, Direction.UP);
                                    break;
                                case this.configManager.getStateName("crawl", pet):
                                    this.updateDirection(
                                        pet,
                                        pet.scaleX === -1
                                            ? Direction.UPSIDELEFT
                                            : Direction.UPSIDERIGHT
                                    );
                                    break;
                                default:
                                    this.petJumpOrPlayRandomState(pet);
                                    break;
                            }
                        }, 50);
                    }
                },
            });

            this.petBeyondScreenSwitchClimb(pet, {
                up: this.getPetBoundTop(pet),
                down: this.getPetBoundDown(pet),
                left: this.getPetBoundLeft(pet),
                right: this.getPetBoundRight(pet),
            });
        });

        this.physics.world.on(
            "worldbounds",
            (
                body: Phaser.Physics.Arcade.Body,
                up: boolean,
                down: boolean,
                left: boolean,
                right: boolean
            ) => {
                const pet = body.gameObject as Pet;
                if (
                    pet.anims &&
                    pet.anims.getName() ===
                        this.configManager.getStateName("crawl", pet)
                ) {
                    if (left || right) {
                        this.petJumpOrPlayRandomState(pet);
                    }
                    return;
                }

                if (up) {
                    if (!this.allowPetClimbing) {
                        this.petJumpOrPlayRandomState(pet);
                        return;
                    }

                    if (pet.availableStates.includes("crawl")) {
                        this.switchState(pet, "crawl");
                        return;
                    }
                    this.petJumpOrPlayRandomState(pet);
                } else if (down) {
                    this.switchStateAfterPetJump(pet);
                    this.petOnTheGroundPlayRandomState(pet);
                }

                this.petBeyondScreenSwitchClimb(pet, {
                    up: up,
                    down: down,
                    left: left,
                    right: right,
                });
            }
        );
    }

    update(_time: number, delta: number): void {
        this.frameCount += delta;

        if (this.frameCount >= this.UPDATE_DELAY) {
            this.frameCount = 0;
            if (this.allowPetInteraction) {
                this.inputManager.checkIsMouseInOnPet();
            }

            this.randomJumpIfPetClimbAndCrawl();
        }
    }

    addPet(sprite: ISpriteConfig, index: number): void {
        this.configManager.registerSpriteStateAnimation(sprite);

        const randomX = Phaser.Math.Between(
            100,
            this.physics.world.bounds.width - 100
        );
        const petY = 0 + this.configManager.getFrameSize(sprite).frameHeight;
        this.pets[index] = this.physics.add
            .sprite(randomX, petY, sprite.name)
            .setInteractive({
                draggable: true,
                pixelPerfect: true,
            }) as Pet;

        this.scalePet(this.pets[index], 1);

        this.pets[index].setCollideWorldBounds(true, 0, 0, true);

        this.pets[index].availableStates = Object.keys(sprite.states);
        this.pets[index].canPlayRandomState = true;
        this.pets[index].canRandomFlip = true;
        this.pets[index].id = sprite.id || sprite.name;

        this.petJumpOrPlayRandomState(this.pets[index]);
    }

    updateDirection(pet: Pet, direction: Direction): void {
        pet.direction = direction;
        this.updateMovement(pet);
    }

    updateStateDirection(pet: Pet, state: string): void {
        let direction = Direction.UNKNOWN;

        switch (state) {
            case "walk":
                direction = pet.scaleX < 0 ? Direction.LEFT : Direction.RIGHT;
                break;
            case "jump":
                this.toggleFlipX(pet);
                direction = Direction.DOWN;
                break;
            case "climb":
                direction = Direction.UP;
                break;
            case "crawl":
                pet.scaleX > 0
                    ? (direction = Direction.UPSIDELEFT)
                    : (direction = Direction.UPSIDERIGHT);
                break;
            default:
                direction = Direction.UNKNOWN;
                break;
        }

        this.updateDirection(pet, direction);
    }

    updateMovement(pet: Pet): void {
        switch (pet.direction) {
            case Direction.RIGHT:
                pet.setVelocity(this.PET_MOVE_VELOCITY, 0);
                pet.setAcceleration(0);
                this.setPetLookToTheLeft(pet, false);
                break;
            case Direction.LEFT:
                pet.setVelocity(-this.PET_MOVE_VELOCITY, 0);
                pet.setAcceleration(0);
                this.setPetLookToTheLeft(pet, true);
                break;
            case Direction.UP:
                pet.setVelocity(0, -this.PET_MOVE_VELOCITY);
                pet.setAcceleration(0);
                break;
            case Direction.DOWN:
                pet.setVelocity(0, this.PET_MOVE_VELOCITY);
                pet.setAcceleration(0, this.PET_MOVE_ACCELERATION);
                break;
            case Direction.UPSIDELEFT:
                pet.setVelocity(-this.PET_MOVE_VELOCITY);
                pet.setAcceleration(0);
                this.setPetLookToTheLeft(pet, true);
                break;
            case Direction.UPSIDERIGHT:
                pet.setVelocity(
                    this.PET_MOVE_VELOCITY,
                    -this.PET_MOVE_VELOCITY
                );
                pet.setAcceleration(0);
                this.setPetLookToTheLeft(pet, false);
                break;
            case Direction.UNKNOWN:
                pet.setVelocity(0);
                pet.setAcceleration(0);
                break;
            default:
                pet.setVelocity(0);
                pet.setAcceleration(0);
                break;
        }

        const isMovingUp = [
            Direction.UP,
            Direction.UPSIDELEFT,
            Direction.UPSIDERIGHT,
        ].includes(pet.direction as Direction);

        // @ts-ignore
        pet.body!.setAllowGravity(!isMovingUp);

        if (pet.direction === Direction.UP) {
            pet.setVelocityX(0);
        }
    }

    switchState(
        pet: Pet,
        state: string,
        options: ISwitchStateOptions = {
            repeat: -1,
            delay: 0,
            repeatDelay: 0,
        }
    ): void {
        try {
            if (!pet.anims) return;

            if (!this.allowPetClimbing) {
                if (state === "climb" || state === "crawl") return;
            }

            const animationKey = this.configManager.getStateName(state, pet);
            if (pet.anims && pet.anims.getName() === animationKey) return;
            if (!pet.availableStates.includes(state)) return;

            pet.anims.play({
                key: animationKey,
                repeat: options.repeat ?? -1,
                delay: options.delay ?? 0,
                repeatDelay: options.repeatDelay ?? 0,
            });

            if (state === "climb" || state === "crawl") {
                this.petClimbAndCrawlIndex.push(this.pets.indexOf(pet));
            } else {
                this.petClimbAndCrawlIndex = this.petClimbAndCrawlIndex.filter(
                    (index) => index !== this.pets.indexOf(pet)
                );
            }

            this.updateStateDirection(pet, state);
        } catch (err: any) {
            console.error("switchState error:", err);
        }
    }

    setPetLookToTheLeft(pet: Pet, lookToTheLeft: boolean): void {
        if (lookToTheLeft) {
            if (pet.scaleX > 0) {
                this.toggleFlipX(pet);
            }
            return;
        }

        if (pet.scaleX < 0) {
            this.toggleFlipX(pet);
        }
    }

    scalePet(pet: Pet, scaleValue: number): void {
        const scaleX = pet.scaleX > 0 ? scaleValue : -scaleValue;
        const scaleY = pet.scaleY > 0 ? scaleValue : -scaleValue;
        pet.setScale(scaleX, scaleY);
    }

    toggleFlipX(pet: Pet): void {
        pet.scaleX > 0 ? pet.setOffset(pet.width, 0) : pet.setOffset(0, 0);
        pet.setScale(pet.scaleX * -1, pet.scaleY);
    }

    toggleFlipXThenUpdateDirection(pet: Pet): void {
        this.toggleFlipX(pet);

        switch (pet.direction) {
            case Direction.RIGHT:
                this.updateDirection(pet, Direction.LEFT);
                break;
            case Direction.LEFT:
                this.updateDirection(pet, Direction.RIGHT);
                break;
            case Direction.UPSIDELEFT:
                this.updateDirection(pet, Direction.UPSIDERIGHT);
                break;
            case Direction.UPSIDERIGHT:
                this.updateDirection(pet, Direction.UPSIDELEFT);
                break;
            default:
                break;
        }
    }

    getOneRandomState(pet: Pet): string {
        let randomStateIndex;

        do {
            randomStateIndex = Phaser.Math.Between(
                0,
                pet.availableStates.length - 1
            );
        } while (
            this.FORBIDDEN_RAND_STATE.includes(
                pet.availableStates[randomStateIndex]
            )
        );

        return pet.availableStates[randomStateIndex];
    }

    playRandomState(pet: Pet): void {
        if (!pet.canPlayRandomState) return;

        this.switchState(pet, this.getOneRandomState(pet));
        pet.canPlayRandomState = false;

        setTimeout(() => {
            pet.canPlayRandomState = true;
        }, this.RAND_STATE_DELAY);
    }

    switchStateAfterPetJump(pet: Pet): void {
        if (!pet) return;
        if (
            pet.anims &&
            pet.anims.getName() !== this.configManager.getStateName("jump", pet)
        )
            return;

        if (pet.availableStates.includes("fall")) {
            this.switchState(pet, "fall", {
                repeat: 0,
            });

            pet.canPlayRandomState = false;
            pet.on("animationcomplete", () => {
                pet.canPlayRandomState = true;
                this.playRandomState(pet);
            });

            return;
        }
        this.playRandomState(pet);
    }

    getPetGroundPosition(pet: Pet): number {
        return (
            this.physics.world.bounds.height -
            pet.height * Math.abs(pet.scaleY) * pet.originY
        );
    }

    getPetTopPosition(pet: Pet): number {
        return pet.height * Math.abs(pet.scaleY) * pet.originY;
    }

    getPetLeftPosition(pet: Pet): number {
        return pet.width * Math.abs(pet.scaleX) * pet.originX;
    }

    getPetRightPosition(pet: Pet): number {
        return (
            this.physics.world.bounds.width -
            pet.width * Math.abs(pet.scaleX) * pet.originX
        );
    }

    getPetBoundDown(pet: Pet): boolean {
        return pet.y >= this.getPetGroundPosition(pet);
    }

    getPetBoundLeft(pet: Pet): boolean {
        return pet.x <= this.getPetLeftPosition(pet);
    }

    getPetBoundRight(pet: Pet): boolean {
        return pet.x >= this.getPetRightPosition(pet);
    }

    getPetBoundTop(pet: Pet): boolean {
        return pet.y <= this.getPetTopPosition(pet);
    }

    updatePetAboveTaskbar(): void {
        const gameWidth = this.scale.width;
        const gameHeight = this.scale.height;

        if (this.allowPetAboveTaskbar) {
            const taskbarHeight =
                window.screen.height - window.screen.availHeight;

            this.physics.world.setBounds(
                0,
                0,
                gameWidth,
                gameHeight - taskbarHeight
            );
            return;
        }

        this.physics.world.setBounds(
            0,
            0,
            gameWidth,
            gameHeight
        );
    }

    petJumpOrPlayRandomState(pet: Pet): void {
        if (!pet) return;

        if (pet.availableStates.includes("jump")) {
            this.switchState(pet, "jump");
            return;
        }

        this.switchState(pet, this.getOneRandomState(pet));
    }

    petOnTheGroundPlayRandomState(pet: Pet): void {
        if (!pet) return;

        switch (pet.anims.getName()) {
            case this.configManager.getStateName("climb", pet):
                return;
            case this.configManager.getStateName("crawl", pet):
                return;
            case this.configManager.getStateName("drag", pet):
                return;
            case this.configManager.getStateName("jump", pet):
                return;
        }

        const random = Phaser.Math.Between(0, 2000);
        if (
            pet.anims &&
            pet.anims.getName() === this.configManager.getStateName("walk", pet)
        ) {
            if (random >= 0 && random <= 5) {
                this.switchState(pet, "idle");
                setTimeout(() => {
                    if (
                        pet.anims &&
                        pet.anims.getName() !==
                            this.configManager.getStateName("idle", pet)
                    )
                        return;
                    this.switchState(pet, "walk");
                }, Phaser.Math.Between(3000, 6000));
                return;
            }
        } else {
            if (random >= 777 && random <= 800) {
                this.playRandomState(pet);
                return;
            }
        }

        if (random >= 888 && random <= 890) {
            if (pet.canRandomFlip) {
                this.toggleFlipXThenUpdateDirection(pet);
                pet.canRandomFlip = false;

                setTimeout(() => {
                    pet.canRandomFlip = true;
                }, this.FLIP_DELAY);
            }
        } else if (random >= 777 && random <= 780) {
            this.playRandomState(pet);
        } else if (random >= 170 && random <= 175) {
            this.switchState(pet, "walk");
        }
    }

    randomJumpIfPetClimbAndCrawl(): void {
        if (this.petClimbAndCrawlIndex.length === 0) return;

        for (const index of this.petClimbAndCrawlIndex) {
            const pet = this.pets[index];
            if (!pet) continue;

            switch (pet.anims.getName()) {
                case this.configManager.getStateName("drag", pet):
                    continue;
                case this.configManager.getStateName("jump", pet):
                    continue;
            }

            const random = Phaser.Math.Between(0, 500);

            if (random === 78) {
                let newPetx = pet.x;
                if (
                    pet.anims &&
                    pet.anims.getName() ===
                        this.configManager.getStateName("climb", pet)
                ) {
                    newPetx =
                        pet.scaleX < 0
                            ? Phaser.Math.Between(pet.x, 500)
                            : Phaser.Math.Between(
                                  pet.x,
                                  this.physics.world.bounds.width - 500
                              );
                }

                if (pet.body!.enable) pet.body!.enable = false;
                this.switchState(pet, "jump");
                this.tweens.add({
                    targets: pet,
                    x: newPetx,
                    y: this.getPetGroundPosition(pet),
                    duration: 3000,
                    ease: Ease.QuadEaseOut,
                    onComplete: () => {
                        if (!pet.body!.enable) {
                            pet.body!.enable = true;
                            this.switchStateAfterPetJump(pet);
                        }
                    },
                });
                return;
            }

            // add random pause when climb
            if (random >= 0 && random <= 5) {
                if (
                    pet.anims &&
                    pet.anims.getName() ===
                        this.configManager.getStateName("climb", pet)
                ) {
                    pet.anims.pause();
                    this.updateDirection(pet, Direction.UNKNOWN);
                    // @ts-ignore
                    pet.body!.allowGravity = false;
                    setTimeout(() => {
                        if (pet.anims && !pet.anims.isPlaying) {
                            pet.anims.resume();
                            this.updateDirection(pet, Direction.UP);
                        }
                    }, Phaser.Math.Between(3000, 6000));
                    return;
                } else if (
                    pet.anims &&
                    pet.anims.getName() ===
                        this.configManager.getStateName("crawl", pet)
                ) {
                    pet.anims.pause();
                    this.updateDirection(pet, Direction.UNKNOWN);
                    // @ts-ignore
                    pet.body!.allowGravity = false;
                    setTimeout(() => {
                        if (pet.anims && !pet.anims.isPlaying) {
                            pet.anims.resume();
                            this.updateDirection(
                                pet,
                                pet.scaleX < 0
                                    ? Direction.UPSIDELEFT
                                    : Direction.UPSIDERIGHT
                            );
                        }
                    }, Phaser.Math.Between(3000, 6000));
                    return;
                }
            }
        }
    }

    petBeyondScreenSwitchClimb(pet: Pet, worldBounding: IWorldBounding): void {
        if (!pet) return;

        switch (pet.anims.getName()) {
            case this.configManager.getStateName("climb", pet):
                return;
            case this.configManager.getStateName("crawl", pet):
                return;
        }

        if (!this.allowPetClimbing) return;

        if (worldBounding.left) {
            if (pet.availableStates.includes("climb")) {
                this.setPetLookToTheLeft(pet, true);
                this.switchState(pet, "climb");
                return;
            }
        } else if (worldBounding.right) {
            if (pet.availableStates.includes("climb")) {
                this.setPetLookToTheLeft(pet, false);
                this.switchState(pet, "climb");
                return;
            }
        }
    }
}
