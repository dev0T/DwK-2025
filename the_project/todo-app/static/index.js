try {
  const response = await fetch('/api/v1/todos', {
    method: 'GET'
  })
  for (const item of await response.json()) {
    createTodoItem(item.title)
  }
} catch (error) {
  let message = `Error fetching todos. ${error}`
  console.log(message)
  displayError(message)
}

http: document
  .getElementById('todo-form')
  .addEventListener('submit', async event => {
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
        let message = 'Error while creating task. Try again.'
        displayError(message)
      }
    } catch (e) {
      let message = `Error creating todo. Error: ${e}`
      displayError(message)
    }
  })

function createTodoItem(todoContent) {
  const newTodoItem = document.createElement('li')
  newTodoItem.textContent = todoContent
  document.getElementById('todo-list').appendChild(newTodoItem)
}

function displayError(content) {
  const errorDisplay = document.getElementById('error-display')
  errorDisplay.replaceChildren()
  const errorWarning = document.createElement('span')
  errorWarning.style.color = 'red'
  errorWarning.innerHTML = content
  errorDisplay.appendChild(errorWarning)
}
