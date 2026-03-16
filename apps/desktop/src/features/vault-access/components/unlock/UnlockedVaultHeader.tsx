import { OpenVaultLogo } from "@/components/icons";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/shadcn/tooltip";
import { truncateLeft } from "@/utils/format";

type UnlockedVaultHeaderProps = {
  path: string;
};

export const UnlockedVaultHeader = ({ path }: UnlockedVaultHeaderProps) => (
  <div className="flex flex-col items-center space-y-12 text-center">
    <div>
      <OpenVaultLogo logoClassName="size-18" hideName />
    </div>

    <div className="space-y-4">
      <h2 className="text-3xl font-semibold tracking-tight text-foreground">
        Unlocking Vault:
      </h2>
      <Tooltip>
        <TooltipTrigger>
          <div className="flex max-w-sm items-center justify-center gap-2">
            <p className="text-base font-medium tracking-wide text-muted-foreground/50">
              {truncateLeft(path, 40)}
            </p>
          </div>
        </TooltipTrigger>
        <TooltipContent>{path}</TooltipContent>
      </Tooltip>
    </div>
  </div>
);
