try {
  const response = await fetch('/api/v1/todos', {
    method: 'GET'
  })
  for (const item of await response.json()) {
    createTodoItem(item.title)
  }
} catch (error) {
  console.log('Error fetching todos!', error)
}

document.getElementById('todo-form').addEventListener('submit', async event => {
  event.preventDefault()
  const errorDisplay = document.getElementById('error-display')
  errorDisplay.replaceChildren()
  const newTodo = document.forms['todo-form']['todo-content'].value
  try {
    const response = await fetch('/api/v1/todos', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ title: newTodo })
    })
    if (response.status === 200) {
      createTodoItem(newTodo)
      document.forms['todo-form']['todo-content'].value = ''
    } else {
      const errorWarning = document.createElement('span')
      errorWarning.innerHTML = 'Error while creating task. Try again.'
      errorDisplay.appendChild(errorWarning)
    }
  } catch (e) {
    console.log('Error creating todo!', e)
  }
})

function createTodoItem(todoContent) {
  const newTodoItem = document.createElement('li')
  newTodoItem.textContent = todoContent
  document.getElementById('todo-list').appendChild(newTodoItem)
}
