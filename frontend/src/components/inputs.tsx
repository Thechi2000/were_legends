import Link, { LinkProps } from "next/link";

export function Button({ children, className, disabled, onClick, ...props }: any) {
  function isDisabled() {
    return disabled !== undefined && disabled;
  }

  return (
    <div
      className={
        "flex justify-between items-center gap-2 border-2 border-sky-400 rounded-md p-1 " +
        (isDisabled() ? "text-slate-400 " : "cursor-pointer ") +
        className
      }
      onClick={() => {
        if (!isDisabled()) {
            onClick()
        }
      }}
      {...props}
    >
      <i className={"arrow-right " + (isDisabled() ? "invisible" : "")} />
      {children}
      <i className={"arrow-left " + (isDisabled() ? "invisible" : "")} />
    </div>
  );
}

export function Href({
  className,
  children,
  arrowLeft,
  arrowRight,
  ...props
}: LinkProps & {
  arrowLeft?: boolean;
  arrowRight?: boolean;
  className?: string;
  children: any;
}) {
  return (
    <div className="flex flex-row gap-2 justify-center items-center">
      {arrowLeft ? <i className="arrow-right" /> : <></>}
      <Link className={"text-purple-300 " + className} {...props}>
        {children}
      </Link>
      {arrowRight ? <i className="arrow-left" /> : <></>}
    </div>
  );
}
