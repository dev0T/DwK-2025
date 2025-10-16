import Checkbox from '@/components/ui/Checkbox/Checkbox'
import styles from './TodoListItem.module.scss'
import { API_URL, fetcher } from '@/lib/consts'
import { Todo } from '@/components/TodoList/TodoList'
import useSWR, { mutate } from 'swr'

interface TodoListItemProps {
  todo: Todo
}

export default function TodoListItem({ todo }: TodoListItemProps) {
  const { isLoading } = useSWR<Todo[]>(`${API_URL}/api/v1/todos`, fetcher)

  const handleCheckbox = async (values: Todo) => {
    try {
      const response = await fetch(`${API_URL}/api/v1/todos`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ ...values, done: !values.done })
      })
      if (response.status === 200) {
        mutate<Todo[]>(`${API_URL}/api/v1/todos`, async todos => [
          ...(todos ?? []),
          await response.json()
        ])
      } else {
        const content = response.body
        console.log(`Content: ${content}`)
      }
    } catch (error: unknown) {
      if (error instanceof Error) {
        console.log('do something?')
      }
      console.log(`Error: ${error}`)
    }
  }

  return (
    <div className={styles.todoItem}>
      <Checkbox
        id={todo.id}
        className={styles.todoItemCheckbox}
        disabled={isLoading}
        checked={todo.done}
        onChange={() => handleCheckbox(todo)}
      />
      <label htmlFor="terms" className={styles.todoItemLabel}>
        {todo.title}
      </label>
    </div>
  )
}
