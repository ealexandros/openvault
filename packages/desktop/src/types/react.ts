export type ClickEvent<TElement = HTMLButtonElement> = React.MouseEvent<TElement>;

export type InputEvent<TElement = HTMLInputElement> = React.ChangeEvent<TElement>;

export type StateSetter<T> = React.Dispatch<React.SetStateAction<T>>;

export type PropsWithClassName<P = unknown> = P & { className?: string };
