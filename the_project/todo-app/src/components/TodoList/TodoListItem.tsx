import Checkbox from '@/components/ui/Checkbox/Checkbox'
import styles from './TodoListItem.module.scss'
import { API_URL } from '@/lib/consts'
import { Todo } from '@/components/TodoList/TodoList'
import useSWRMutation from 'swr/mutation'

interface TodoListItemProps {
  todo: Todo
}

const updateTodo = async (url: string, { arg }: { arg: Todo }) => {
  return fetch(`${url}/${arg.id}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ ...arg, done: !arg.done })
  }).then(res => res.json())
}

export default function TodoListItem({ todo }: TodoListItemProps) {
  const { trigger, isMutating } = useSWRMutation(
    `${API_URL}/api/v1/todos`,
    updateTodo
  )

  const handleCheckbox = async (todo: Todo) => {
    try {
      const result = await trigger(todo)
      console.log(result)
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
        disabled={isMutating}
        checked={todo.done}
        onChange={() => handleCheckbox(todo)}
      />
      <label htmlFor="terms" className={styles.todoItemLabel}>
        {todo.title}
      </label>
    </div>
  )
}
