'use client'
import { useState } from 'react'
import { Controller, useForm } from 'react-hook-form'
import { z } from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'
import { mutate } from 'swr'
import InputGroup from '@/components/ui/Input/InputGroup'
import Button from '@/components/ui/Button/Button'
import { Todo } from '@/components/TodoList/TodoList'

import styles from './CreateTodo.module.scss'

const newTodoSchema = z.object({
  title: z.string().max(140, {
    message: 'Todo text can only have up to 140 characters.'
  })
})

type newTodo = z.infer<typeof newTodoSchema>

export default function CreateTodo() {
  const form = useForm<newTodo>({
    resolver: zodResolver(newTodoSchema),
    defaultValues: {
      title: ''
    }
  })
  const [errorMessage] = useState<string>('')

  const onSubmit = async (values: newTodo) => {
    try {
      const response = await fetch(`/api/v1/todos`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(values)
      })
      if (response.status === 200) {
        mutate<Todo[]>(`/api/v1/todos`, async todos => [
          ...(todos ?? []),
          await response.json()
        ])
      } else {
        const content = response.body
        console.log(`Content: ${content}`)
      }
    } catch (error: unknown) {
      if (error instanceof Error) {
        form.setError('root', {
          type: 'serverSideError',
          message: error.message
        })
      }
      console.log(`Error: ${error}`)
    }
  }

  return (
    <form onSubmit={form.handleSubmit(onSubmit)} className={styles.form}>
      <div className={styles.formInputsContainer}>
        <Controller
          name="title"
          control={form.control}
          render={({ field }) => (
            <InputGroup
              id="test-input"
              type="text"
              message={errorMessage}
              inputProps={{
                type: 'text',
                placeholder: 'Placeholder',
                maxLength: 140,
                ...field
              }}
            />
          )}
        />
        <div>
          <Button type="submit" disabled={form.formState.isSubmitting}>
            Create
          </Button>
          <div style={{ minHeight: '25px' }}></div>
        </div>
      </div>
    </form>
  )
}
