import { ChangeEvent, useState } from 'react'
import { invoke } from '@tauri-apps/api'

function App() {
  const [message, setMessage] = useState("Hello")
  const [name, setName] = useState("World")

  const handleClick = (name: string) => {
    invoke('hello', { name: name })
      .then((msg: any) => setMessage(msg))
  }

  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value)
  }

  return (
    <div className="App">
      <h1>{message}</h1>
      <input type="text" value={name} onChange={handleChange} />
      <button onClick={() => handleClick(name)}>
        set name
      </button>
    </div>
  )
}

export default App
