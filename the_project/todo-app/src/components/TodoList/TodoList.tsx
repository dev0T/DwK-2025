'use client'

import TodoListItem from '@/components/TodoList/TodoListItem'
import React from 'react'
import useSWR from 'swr'
import Spinner from '@/components/ui/Spinner/Spinner'

import styles from './TodoList.module.scss'

export interface Todo {
  id: string
  title: string
}

export default function TodoList() {
  const fetcher = (url: string) => fetch(url).then(r => r.json())

  const { data: todos, isLoading } = useSWR<Todo[]>(`/api/v1/todos`, fetcher)

  return (
    <div className={styles.todoListContainer}>
      {isLoading ? (
        <Spinner className={styles.todoListSpinner} />
      ) : (
        <>
          {todos?.map((todo: Todo) => (
            <TodoListItem key={todo.id} content={todo.title} />
          ))}
        </>
      )}
    </div>
  )
}
