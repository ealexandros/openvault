"use client";

import { m, MotionProps } from "framer-motion";

const motionProps: MotionProps = {
  initial: { opacity: 0, x: 20 },
  animate: { opacity: 1, x: 0 },
  exit: { opacity: 0, x: -20 },
  transition: { duration: 0.3, ease: "easeOut" },
};

export const StepContainer = ({ children }: { children: React.ReactNode }) => (
  <m.div {...motionProps} className="space-y-8">
    {children}
  </m.div>
);
