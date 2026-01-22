import type { Node, Edge } from '@vue-flow/core';

export interface ActivityNode extends Node {
  data: {
    label: string;
    eventType: 'app_focus' | 'keyboard' | 'mouse' | 'idle';
    time: string;
    app?: string;
    windowTitle?: string;
    keyCount?: number;
    mouseDistance?: number;
    clickCount?: number;
  };
}

export interface ActivityEdge extends Edge {
  data?: {
    duration?: string;
  };
}
