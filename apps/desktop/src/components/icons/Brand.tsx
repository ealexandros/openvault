import { cn } from "@/utils/cn";

type BrandProps = {
  logoSize?: number;
  logoClassName?: string;
  nameClassName?: string;
  wrapperClassName?: string;
  hideLogo?: boolean;
  hideName?: boolean;
};

export const Brand = ({
  logoSize,
  nameClassName,
  logoClassName,
  wrapperClassName,
  hideLogo = false,
  hideName = false,
}: BrandProps) => (
  <span className={cn("flex items-center space-x-1", wrapperClassName)}>
    {!hideLogo && (
      <svg
        viewBox="0 0 242 297"
        xmlns="http://www.w3.org/2000/svg"
        className={cn("size-9", logoClassName)}
        {...(logoSize != null && { width: `${logoSize}px !important` })}>
        <path
          d="M120.851 111.158C132.324 111.158 141.625 120.446 141.625 131.904C141.625 139.272 137.778 145.744 131.98 149.424L135.332 170.06C135.332 173.538 132.509 176.358 129.026 176.358H112.675C109.192 176.358 106.369 173.538 106.369 170.06L109.721 149.424C103.923 145.744 100.076 139.272 100.076 131.904C100.076 120.446 109.377 111.158 120.851 111.158Z"
          fill="#3292E4"
        />
        <path
          d="M228.534 45.269C236.768 49.2453 242 57.5827 242 66.7261V188.42C242 212.784 206.893 253.688 148.027 289.047C142.151 292.576 134.798 288.265 134.798 281.41V254.945C134.798 252.179 136.074 249.57 138.235 247.844C171.517 221.258 191.304 197.776 191.304 179.21V87.122C191.304 78.548 186.943 70.5623 179.729 65.9283L139.44 43.6345C136.576 42.0493 134.798 39.0336 134.798 35.7597V14.341C134.798 7.70452 141.736 3.35055 147.712 6.23663L228.534 45.269Z"
          fill="url(#paint0_linear_308_1636)"
        />
        <path
          d="M106.885 35.9782C106.885 39.2628 105.096 42.2866 102.216 43.8674L62.0177 65.9369C54.7674 70.5638 50.379 78.5704 50.379 87.1712V179.21C50.379 198.331 69.9915 222.462 103.3 248.072C105.545 249.799 106.885 252.462 106.885 255.294V281.63C106.885 288.401 99.6932 292.722 93.8287 289.337C34.8395 255.293 0.000263787 213.527 0 188.42V66.7686C6.49223e-05 57.6032 5.25725 49.2503 13.5209 45.2856L93.992 6.67773C99.9664 3.81137 106.885 8.16572 106.885 14.7922V35.9782Z"
          fill="url(#paint1_linear_308_1636)"
        />
        <defs>
          <linearGradient
            id="paint0_linear_308_1636"
            x1="237.273"
            y1="-12.1119"
            x2="1.65855"
            y2="305.018"
            gradientUnits="userSpaceOnUse">
            <stop stopColor="#5FB6EC" />
            <stop offset="1" stopColor="#2E8CE4" />
          </linearGradient>
          <linearGradient
            id="paint1_linear_308_1636"
            x1="237.273"
            y1="-12.1119"
            x2="1.65855"
            y2="305.018"
            gradientUnits="userSpaceOnUse">
            <stop stopColor="#5FB6EC" />
            <stop offset="1" stopColor="#2E8CE4" />
          </linearGradient>
        </defs>
      </svg>
    )}
    {!hideName && (
      <span
        className={cn(
          "text-2xl font-bold tracking-wide text-gray-700 uppercase",
          nameClassName,
        )}>
        Open<span className="text-primary">Vault</span>
      </span>
    )}
  </span>
);
