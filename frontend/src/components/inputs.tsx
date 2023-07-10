import Link, { LinkProps } from "next/link";

export function Button({ children, className, ...props }: any) {
  return (
    <div
      className={
        "flex justify-between items-center gap-2 border-2 border-sky-400 rounded-md p-1 cursor-pointer " +
        className
      }
      {...props}
    >
      <i className="arrow-right" />
      {children}
      <i className="arrow-left" />
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
