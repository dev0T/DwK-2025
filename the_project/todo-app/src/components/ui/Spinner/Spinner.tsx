import { PropsWithClassName } from '@/lib/utils'
import styles from './Spinner.module.scss'
import clsx from 'clsx'

// https://stephaniehobson.ca/posts/css-loading-spinner/

type SpinnerProps = PropsWithClassName

export default function Spinner({ className }: SpinnerProps) {
  return (
    <div className={clsx(styles.spinner, className)}>
      <span></span>
      <span></span>
      <span></span>
      <span></span>
      <span></span>
      <span></span>
      <span></span>
      <span></span>
    </div>
  )
}
