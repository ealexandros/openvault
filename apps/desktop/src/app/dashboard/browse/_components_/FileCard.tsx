import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/shadcn/dropdown-menu";
import { cn } from "@/utils/cn";
import { FileTextIcon, FolderIcon, MoreVerticalIcon, Trash2Icon } from "lucide-react";

type FileItem = {
  id: string;
  name: string;
  type: "file" | "folder";
  details?: string;
  children?: FileItem[];
};

type FileCardProps = {
  item: FileItem;
  onClick: () => void;
  onDelete: () => void;
};

export const FileCard = ({ item, onClick, onDelete }: FileCardProps) => {
  const isFolder = item.type === "folder";

  return (
    <div
      onClick={onClick}
      className={cn(
        "group relative flex cursor-pointer flex-col gap-3 overflow-hidden rounded-2xl border p-4 transition-all duration-300",
        isFolder
          ? "border-primary/10 bg-linear-to-br from-primary/5 to-transparent hover:border-primary/30 hover:shadow-lg hover:shadow-primary/5"
          : "border-border/40 bg-card hover:border-border hover:bg-muted/30 hover:shadow-md",
      )}>
      <div className="flex items-start justify-between">
        <div
          className={cn(
            "rounded-xl border p-2.5 transition-all duration-300",
            isFolder
              ? "border-primary/20 bg-primary/10 text-primary group-hover:scale-110 group-hover:rotate-3"
              : "border-border/50 bg-muted/50 text-muted-foreground group-hover:border-foreground/10 group-hover:bg-background group-hover:text-foreground",
          )}>
          {isFolder ? <FolderIcon className="size-5" /> : <FileTextIcon className="size-5" />}
        </div>

        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <button
              onClick={e => e.stopPropagation()}
              className="rounded-lg p-1.5 text-muted-foreground opacity-0 transition-opacity group-hover:opacity-100 hover:bg-foreground/5 hover:text-foreground">
              <MoreVerticalIcon className="size-4" />
            </button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem
              variant="destructive"
              onClick={e => {
                e.stopPropagation();
                onDelete();
              }}>
              <Trash2Icon className="mr-2 size-3.5" />
              Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>

      <div className="space-y-0.5">
        <p className="truncate text-sm font-semibold text-foreground/90 transition-colors group-hover:text-foreground">
          {item.name}
        </p>
        <p className="text-[10px] font-medium tracking-wider text-muted-foreground/70 uppercase">
          {isFolder ? `${item.children?.length ?? 0} items` : item.details}
        </p>
      </div>

      <div
        className={cn(
          "pointer-events-none absolute inset-0 opacity-0 transition-opacity duration-500 group-hover:opacity-100",
          isFolder
            ? "bg-linear-to-tr from-primary/5 via-transparent to-transparent"
            : "bg-linear-to-tr from-white/5 via-transparent to-transparent",
        )}
      />
    </div>
  );
};
