import IpsumImage from '@/components/IpsumImage/IpsumImage'
import CreateTodo from '@/components/CreateTodo/CreateTodo'
import TodoList from '@/components/TodoList/TodoList'

import styles from './page.module.scss'

export default function Home() {
  return (
    <main className={styles.main}>
      <div className={styles.mainColumns}>
        <IpsumImage />
        <CreateTodo />
        <TodoList />
      </div>
    </main>
  )
}
