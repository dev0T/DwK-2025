import type {
  ComponentPropsWithRef,
  MouseEventHandler,
  PropsWithChildren
} from 'react'
import clsx from 'clsx'

import styles from './Button.module.scss'
import { PropsWithClassName } from '@/lib/utils'

type ButtonProps = {
  onClick?: MouseEventHandler<HTMLButtonElement>
  disabled?: boolean
  title?: string
} & ComponentPropsWithRef<'button'>

const Button = ({
  children,
  className,
  onClick,
  disabled,
  title,
  ...rest
}: PropsWithChildren<PropsWithClassName<ButtonProps>>) => {
  return (
    <button
      {...rest}
      title={title}
      disabled={disabled}
      className={clsx(styles.button, className)}
      onClick={onClick}
    >
      {children}
    </button>
  )
}

export default Button
