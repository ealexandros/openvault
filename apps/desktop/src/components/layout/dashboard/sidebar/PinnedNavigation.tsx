"use client";

import { usePinNavigation } from "../usePinNavigation";
import { NavigationGroup } from "./navigation-group/NavigationGroup";
import { NavigationItem } from "./Sidebar";

type PinnedNavigationProps = {
  items: NavigationItem[];
};

export const PinnedNavigation = ({ items }: PinnedNavigationProps) => {
  const { pinnedRouteUrls } = usePinNavigation();

  const pinnedItems = pinnedRouteUrls
    .map(url => items.find(item => item.url === url))
    .filter((item): item is NavigationItem => item != null);

  if (pinnedItems.length === 0) {
    return null;
  }

  return <NavigationGroup items={pinnedItems} title="Pinned" pinnable={true} />;
};
