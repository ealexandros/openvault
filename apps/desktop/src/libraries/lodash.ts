import { clamp, round } from "lodash-es";

export const lodash = {
  clamp,
  round,

  percent: (value: number, total: number) => {
    if (total <= 0) return 0;
    return clamp((value / total) * 100, 0, 100);
  },
};
