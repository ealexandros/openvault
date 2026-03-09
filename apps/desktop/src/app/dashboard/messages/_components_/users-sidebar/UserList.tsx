import { MessageUserProfile } from "../../useMessagesPage";
import { UserListItem } from "./UserListItem";

type UserListProps = {
  users: MessageUserProfile[];
  selectedUserId: string;
  setSelectedUserId: (id: string) => void;
};

export const UserList = ({ users, selectedUserId, setSelectedUserId }: UserListProps) => (
  <>
    {users.map(user => (
      <UserListItem
        key={user.id}
        user={user}
        selected={selectedUserId === user.id}
        onClick={() => setSelectedUserId(user.id)}
      />
    ))}
  </>
);
