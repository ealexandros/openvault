import {
  ArchiveIcon,
  BookOpenIcon,
  BriefcaseIcon,
  CameraIcon,
  CodeIcon,
  FolderIcon,
  ImageIcon,
  MusicIcon,
  ShieldIcon,
  StarIcon,
  type LucideIcon,
  type LucideProps,
} from "lucide-react";

export const ICON_MAP = {
  folder: FolderIcon,
  star: StarIcon,
  briefcase: BriefcaseIcon,
  archive: ArchiveIcon,
  "book-open": BookOpenIcon,
  image: ImageIcon,
  music: MusicIcon,
  camera: CameraIcon,
  code: CodeIcon,
  shield: ShieldIcon,
} as const satisfies Record<string, LucideIcon>;

export type FolderIconName = keyof typeof ICON_MAP;

export const FOLDER_ICON_OPTIONS = Object.keys(ICON_MAP).map(name => ({
  name: name as FolderIconName,
  Icon: ICON_MAP[name as FolderIconName],
}));

export const renderFolderIcon = (iconName: string, props?: LucideProps) => {
  let Icon = FolderIcon;

  if (iconName in ICON_MAP) {
    Icon = ICON_MAP[iconName as FolderIconName];
  }

  return <Icon {...props} />;
};
