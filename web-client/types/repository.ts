export type Repository = {
  id: string;
  name: string;
  url: string;
  isEnabled: boolean;
  gitSource: string;
  gitSourceSecretKey?: string;
  gitTarget: string;
  gitTargetSecretKey: string;
  gitClonePeriodSeconds: number;
  lastCloneAt: Date;
  createdAt: Date;
  updatedAt: Date;
};
