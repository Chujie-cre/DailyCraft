import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

export class InputManager {
    private input: Phaser.Input.InputPlugin | undefined;
    private isIgnoreCursorEvents: boolean = false;

    private readonly IGNORE_CURSOR_EVENTS_DELAY: number = 50;

    public setInputManager({ input }: { input: Phaser.Input.InputPlugin }) {
        this.input = input;
    }

    public checkIsMouseInOnPet(): void {
        try {
            invoke("get_mouse_position").then((event: any) => {
                if (event && this.detectMouseOverPet(event.clientX, event.clientY)) {
                    this.turnOffIgnoreCursorEvents();
                    return;
                }
                this.turnOnIgnoreCursorEvents();
            });
        } catch (error) {
            console.log("Error in InputManager checkIsMouseInOnPet()", error);
        }
    }

    public turnOffIgnoreCursorEvents(): void {
        try {
            if (this.isIgnoreCursorEvents) {
                const appWindow = getCurrentWebviewWindow();
                appWindow.setIgnoreCursorEvents(false).then(() => {
                    this.isIgnoreCursorEvents = false;
                });
            }
        } catch (error) {
            console.log("Error in InputManager turnOffIgnoreCursorEvents()", error);
        }
    }

    public turnOnIgnoreCursorEvents(): void {
        try {
            if (!this.isIgnoreCursorEvents) {
                setTimeout(() => {
                    const appWindow = getCurrentWebviewWindow();
                    appWindow.setIgnoreCursorEvents(true).then(() => {
                        this.isIgnoreCursorEvents = true;
                    });
                }, this.IGNORE_CURSOR_EVENTS_DELAY);
            }
        } catch (error) {
            console.log("Error in InputManager turnOnIgnoreCursorEvents()", error);
        }
    }

    private detectMouseOverPet(clientX: number, clientY: number): boolean {
        try {
            if (!this.input) return false;

            // divide by devicePixelRatio because the game world is not scaled
            this.input.mousePointer.x = clientX / window.devicePixelRatio;
            this.input.mousePointer.y = clientY / window.devicePixelRatio;

            return (
                this.input.hitTestPointer(this.input.activePointer).length > 0
            );
        } catch (error) {
            console.log("Error in InputManager detectMouseOverPet()", error);
            return false;
        }
    }
}
