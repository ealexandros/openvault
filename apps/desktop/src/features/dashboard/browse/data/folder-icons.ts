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
} from "lucide-react";

export const FOLDER_ICONS = {
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

export type FolderIconName = keyof typeof FOLDER_ICONS;

export const FOLDER_ICON_OPTIONS = Object.entries(FOLDER_ICONS).map(([name, Icon]) => ({
  name: name as FolderIconName,
  Icon,
}));

export const getFolderIcon = (name: string) => {
  return name in FOLDER_ICONS ? FOLDER_ICONS[name as FolderIconName] : FolderIcon;
};
