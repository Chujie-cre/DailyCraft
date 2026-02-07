import { ISpriteConfig } from "../types/ISpriteConfig";

// 默认桌宠配置 - Pusheen猫
// 精灵图需要放在 public/pets/ 目录下
const defaultPetConfig: ISpriteConfig = {
    name: "Pusheen",
    imageSrc: "pets/Pusheen.png",
    frameSize: 128,
    states: {
        stand: {
            spriteLine: 1,
            frameMax: 1,
        },
        walk: {
            spriteLine: 2,
            frameMax: 4,
        },
        sit: {
            spriteLine: 3,
            frameMax: 1,
        },
        idle: {
            spriteLine: 4,
            frameMax: 8,
        },
        crawl: {
            spriteLine: 8,
            frameMax: 8,
        },
        climb: {
            spriteLine: 9,
            frameMax: 8,
        },
        jump: {
            spriteLine: 5,
            frameMax: 1,
        },
        fall: {
            spriteLine: 6,
            frameMax: 3,
        },
        drag: {
            spriteLine: 7,
            frameMax: 1,
        },
    },
};

export default defaultPetConfig;
