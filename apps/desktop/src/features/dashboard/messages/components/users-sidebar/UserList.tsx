import { SidebarMenu } from "@/components/ui/shadcn/sidebar";
import { type MessageContact } from "@/types/messages";
import { UserListItem } from "./UserListItem";
import { UserListItemSkeleton } from "./UserListItemSkeleton";

type UserListProps = {
  users: MessageContact[];
  selectedUserId: string;
  isLoading?: boolean;
  setSelectedUserId: (id: string) => void;
  onRename: (id: string, newName: string) => Promise<void>;
  onDelete: (id: string) => Promise<void>;
};

export const UserList = ({
  users,
  selectedUserId,
  isLoading,
  setSelectedUserId,
  onRename,
  onDelete,
}: UserListProps) => {
  if (isLoading === true) {
    return (
      <SidebarMenu>
        <UserListItemSkeleton />
        <UserListItemSkeleton />
        <UserListItemSkeleton />
      </SidebarMenu>
    );
  }

  return (
    <SidebarMenu>
      {users.map(user => (
        <UserListItem
          key={user.id}
          user={user}
          selected={selectedUserId === user.id}
          onClick={() => setSelectedUserId(user.id)}
          onRename={onRename}
          onDelete={onDelete}
        />
      ))}
    </SidebarMenu>
  );
};
