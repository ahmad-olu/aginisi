import { io, Socket } from 'socket.io-client'

const socket = io('http://127.0.0.1:8090/socket', {
  //  path: "/socket", // if you used io.ns("/socket", ...)
  transports: ['websocket'],
})

socket.on('connect', () => {
  console.log('✅ Connected:', socket.id)

  socket.emit('to-user', {
    run: 'run',
    away: 'away',
  })
})

socket.on('ping', (data) => {
  console.log('📩 Ping Received:', data)
})

socket.on('user', (data) => {
  console.log('📩 User Received:', data)
})

socket.emit('to-user', {
  run: 'run',
  away: 'away',
})

socket.on('to-user', (data) => {
  console.log('📩 To-User Received:', data)
})

socket.on('disconnect', (reason) => {
  console.log('❌ Disconnected:', reason)
})
