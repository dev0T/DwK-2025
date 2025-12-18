'use client'

import TodoListItem from '@/components/TodoList/TodoListItem'
import React, { useMemo } from 'react'
import useSWR from 'swr'
import Spinner from '@/components/ui/Spinner/Spinner'

import styles from './TodoList.module.scss'
import { API_URL, fetcher } from '@/lib/consts'

export interface Todo {
  id: string
  title: string
  done: boolean
}

export default function TodoList() {
  const { data: todos, isLoading } = useSWR<Todo[]>(
    `${API_URL()}/api/v1/todos`,
    fetcher
  )

  const todosMap = useMemo(() => {
    const map = new Map()
    map.set('done', [])
    map.set('todo', [])

    return todos?.reduce((accumulator, current) => {
      if (current.done) {
        accumulator.set('done', [...map.get('done'), current])
      } else {
        accumulator.set('todo', [...map.get('todo'), current])
      }
      return accumulator
    }, map)
  }, [todos])

  return (
    <div className={styles.todoListContainer}>
      {isLoading ? (
        <Spinner className={styles.todoListSpinner} />
      ) : (
        <>
          <h4>Todo</h4>
          {todosMap?.get('todo')?.length === 0 && (
            <p>Everything done, congrats!</p>
          )}
          {todosMap
            ?.get('todo')
            ?.map((todo: Todo) => <TodoListItem key={todo.id} todo={todo} />)}
          <h4>Done</h4>
          {todosMap
            ?.get('done')
            ?.map((todo: Todo) => <TodoListItem key={todo.id} todo={todo} />)}
        </>
      )}
    </div>
  )
}
