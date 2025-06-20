import { PropsWithClassName } from '@/lib/utils'
import TextInput, { TextInputProps } from '@/components/ui/TextInput/TextInput'
import clsx from 'clsx'

import styles from './InputGroup.module.scss'

type InputGroupProps = PropsWithClassName<{
  label?: string
  message?: string
  id: string
  type: string
  inputProps?: TextInputProps
}>

const InputGroup = ({
  id,
  message,
  className,
  inputProps,
  ...props
}: InputGroupProps) => {
  return (
    <div {...props} className={clsx(styles.inputGroup, className)} tabIndex={0}>
      <TextInput {...inputProps} id={id} />
      <p className={styles.inputMessage}>{message}</p>
    </div>
  )
}

export default InputGroup
