import Checkbox from '@/components/ui/Checkbox/Checkbox'
import styles from './TodoListItem.module.scss'

interface TodoListItemProps {
  content: string
}

export default function TodoListItem({ content }: TodoListItemProps) {
  return (
    <div className={styles.todoItem}>
      <Checkbox id={content} className={styles.todoItemCheckbox} />
      <label htmlFor="terms" className={styles.todoItemLabel}>
        {content}
      </label>
    </div>
  )
}
