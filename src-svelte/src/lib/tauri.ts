export interface SidecarMessageOptions {
  sidecar: boolean;
}

export interface SidecarMessage {
  cmd: string;
  program: string;
  args: string[];
  options: SidecarMessageOptions;
  onEventFn: number;
}

export interface SidecarArgs {
  message: SidecarMessage;
}
