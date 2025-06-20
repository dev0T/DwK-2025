import { PropsWithClassName } from '@/lib/utils'
import React, { ComponentPropsWithRef } from 'react'

type CheckboxProps = PropsWithClassName<{
  title?: string
  disabled?: boolean
  checked?: boolean
  name?: string
}> &
  ComponentPropsWithRef<'input'>

export default function Checkbox({ title, ...props }: CheckboxProps) {
  return (
    <input {...props} type="checkbox">
      {title}
    </input>
  )
}
