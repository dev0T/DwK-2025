import clsx from 'clsx'
import type { ComponentPropsWithRef } from 'react'

import styles from './TextInput.module.scss'
import { PropsWithClassName } from '@/lib/utils'

export type TextInputProps = PropsWithClassName<ComponentPropsWithRef<'input'>>

const TextInput = ({
  value,
  placeholder,
  className,
  type,
  ref,
  ...props
}: TextInputProps) => {
  return (
    <input
      ref={ref}
      className={clsx(styles.textInput, className)}
      {...props}
      type={type}
      value={value}
      placeholder={placeholder}
    />
  )
}

export default TextInput
