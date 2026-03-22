<template>
  <div
    class="window"
    :class="{ 'is-dragging': dragging }"
    :style="{ left: pos.x + 'px', top: pos.y + 'px', width: size.w + 'px' }"
    ref="windowEl"
  >
    <!-- Title Bar -->
    <div class="title-bar" @mousedown.prevent="startDrag" @touchstart.prevent="startDragTouch">
      <div class="title-bar-dots">
        <div class="dot dot-close" />
        <div class="dot dot-min" />
        <div class="dot dot-max" />
      </div>
      <span class="title-bar-label">Command Prompt</span>
    </div>

    <!-- Log List -->
      <div class="log-list" ref="logListEl">
        <template v-if="entries.length">
          <div class="log-entry" v-for="(e, i) in [...entries].reverse()" :key="e.id">
            <span class="entry-text">{{ directory }}> {{ e.text }}</span>
          </div>
        </template>
        <div v-else class="empty-state">
          
        </div>
      </div>

    <div class="divider" />

    <!-- Body -->
    <div class="window-body">
      <!-- Input Row -->
      <div class="input-row">
        <span class="directory">{{ directory }}></span>
        <input
          class="text-input"
          v-model="currentInput"
          @keydown.enter="submitInput"
          ref="inputEl"
          spellcheck="false"
          autocomplete="off"
        />
        <!-- <button class="submit-btn" @click="submitInput">LOG</button> -->
      </div>
    </div>

    <!-- Resize Handle -->
    <div class="resize-handle" @mousedown.prevent="startResize" />
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted, onUnmounted, nextTick } from 'vue'

const windowEl  = ref(null)
const inputEl   = ref(null)
const logListEl = ref(null)
const directory = ref("C:/")

const pos  = ref({ x: 0, y: 0 })
const size = ref({ w: 520 })

onMounted(() => {
  pos.value = {
    x: Math.max(0, (window.innerWidth  - size.value.w) / 2),
    y: Math.max(0, (window.innerHeight - 420) / 2),
  }
})

// Drag
const dragging = ref(false)
let dragOffset = { x: 0, y: 0 }

function startDrag(e) {
  dragging.value = true
  dragOffset = { x: e.clientX - pos.value.x, y: e.clientY - pos.value.y }
  window.addEventListener('mousemove', onDragMove)
  window.addEventListener('mouseup',   onDragEnd)
}

function startDragTouch(e) {
  const t = e.touches[0]
  dragging.value = true
  dragOffset = { x: t.clientX - pos.value.x, y: t.clientY - pos.value.y }
  window.addEventListener('touchmove', onDragMoveTouch, { passive: false })
  window.addEventListener('touchend',  onDragEnd)
}

function onDragMove(e) {
  pos.value = {
    x: Math.max(0, Math.min(window.innerWidth  - size.value.w, e.clientX - dragOffset.x)),
    y: Math.max(0, Math.min(window.innerHeight - 60,           e.clientY - dragOffset.y)),
  }
}

function onDragMoveTouch(e) {
  e.preventDefault()
  const t = e.touches[0]
  pos.value = {
    x: Math.max(0, Math.min(window.innerWidth  - size.value.w, t.clientX - dragOffset.x)),
    y: Math.max(0, Math.min(window.innerHeight - 60,           t.clientY - dragOffset.y)),
  }
}

function onDragEnd() {
  dragging.value = false
  window.removeEventListener('mousemove', onDragMove)
  window.removeEventListener('mouseup',   onDragEnd)
  window.removeEventListener('touchmove', onDragMoveTouch)
  window.removeEventListener('touchend',  onDragEnd)
}

// Resize
let resizeStartX = 0
let resizeStartW = 0

function startResize(e) {
  resizeStartX = e.clientX
  resizeStartW = size.value.w
  window.addEventListener('mousemove', onResizeMove)
  window.addEventListener('mouseup',   onResizeEnd)
}

function onResizeMove(e) {
  size.value.w = Math.max(360, resizeStartW + (e.clientX - resizeStartX))
}

function onResizeEnd() {
  window.removeEventListener('mousemove', onResizeMove)
  window.removeEventListener('mouseup',   onResizeEnd)
}

// Log
const currentInput = ref('')
const entries      = ref([])
let idCounter = 0

function now() {
  return new Date().toLocaleTimeString('en-US', { hour12: false })
}

async function submitInput() {
  const text = currentInput.value.trim()
  if (!text) return
  if (text === "clear"){
    entries.value = ([])
      currentInput.value = ''
    return
  }
  const ans = await invoke("parse_command", {line: text})

  entries.value.push({ id: ++idCounter, text: `${text}\n${ans}`})
  currentInput.value = ''
  nextTick(() => { if (logListEl.value) logListEl.value.scrollTop = 0 })
}

onUnmounted(() => {
  window.removeEventListener('mousemove', onDragMove)
  window.removeEventListener('mouseup',   onDragEnd)
  window.removeEventListener('touchmove', onDragMoveTouch)
  window.removeEventListener('touchend',  onDragEnd)
  window.removeEventListener('mousemove', onResizeMove)
  window.removeEventListener('mouseup',   onResizeEnd)
})
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=VT323&family=Share+Tech+Mono&display=swap');

.window {
  position: fixed;
  min-width: 360px;
  background: #0e1a12;
  border: 1px solid #1f4a2a;
  border-radius: 4px;
  box-shadow:
    0 0 0 1px #0a2e14,
    0 8px 40px rgba(0, 0, 0, 0.8),
    0 0 60px rgba(0, 255, 100, 0.04),
    inset 0 0 0 1px rgba(0, 255, 100, 0.04);
  font-family: 'Share Tech Mono', monospace;
  user-select: none;
  transition: box-shadow 0.15s ease;
}

.window.is-dragging {
  box-shadow:
    0 0 0 1px #1f4a2a,
    0 24px 80px rgba(0, 0, 0, 0.9),
    0 0 80px rgba(0, 255, 100, 0.08),
    inset 0 0 0 1px rgba(0, 255, 100, 0.06);
  cursor: grabbing;
}

/* Title Bar */
.title-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 10px;
  height: 34px;
  background: linear-gradient(135deg, #0f2318 0%, #0a1a10 100%);
  border-bottom: 1px solid #1a3d22;
  border-radius: 3px 3px 0 0;
  cursor: grab;
}

.title-bar:active { cursor: grabbing; }

.title-bar-dots { display: flex; gap: 6px; flex-shrink: 0; }

.dot {
  width: 11px;
  height: 11px;
  border-radius: 50%;
  border: 1px solid rgba(0, 0, 0, 0.3);
  transition: filter 0.15s;
}

.dot-close { background: #3a1414; border-color: #5c2020; }
.dot-min   { background: #2a2a14; border-color: #4a4a20; }
.dot-max   { background: #143a14; border-color: #206020; }

.window:hover .dot-close { background: #c0392b; border-color: #e74c3c; }
.window:hover .dot-min   { background: #d4a017; border-color: #f0b429; }
.window:hover .dot-max   { background: #27ae60; border-color: #2ecc71; }

.title-bar-label {
  flex: 1;
  text-align: center;
  font-family: 'VT323', monospace;
  font-size: 18px;
  color: rgba(0, 255, 100, 0.55);
  letter-spacing: 3px;
  text-transform: uppercase;
  margin-right: 29px;
  pointer-events: none;
}

/* Body */
.window-body {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

/* Input Row */
.input-row { display: flex; gap: 8px; align-items: center; }

.directory {
  background: transparent;
  border: none;
  color: #00ff64;
  font-family: 'Share Tech Mono', monospace;
  font-size: 14px;
  outline: none;
  caret-color: #00ff64;
  letter-spacing: 0.5px;
}

.text-input {
  flex: 1;
  background: transparent;
  border: none;
  border-bottom: 1px solid #1f4a2a;
  color: #00ff64;
  font-family: 'Share Tech Mono', monospace;
  font-size: 14px;
  padding: 6px 4px;
  outline: none;
  caret-color: #00ff64;
  letter-spacing: 0.5px;
  transition: border-color 0.2s;
}

.text-input::placeholder { color: rgba(0, 255, 100, 0.25); }
.text-input:focus { border-bottom-color: #00ff64; }

/* Divider */
.divider {
  height: 1px;
  background: linear-gradient(90deg, transparent, #1f4a2a 20%, #1f4a2a 80%, transparent);
}

/* Log List */
.log-list {
  height: 260px;
  overflow-y: auto;
  display: flex;
  flex-direction: column-reverse;
  
  gap: 4px;
  padding-right: 4px;
}

.log-list::-webkit-scrollbar { width: 4px; }
.log-list::-webkit-scrollbar-track { background: transparent; }
.log-list::-webkit-scrollbar-thumb { background: #1f4a2a; border-radius: 2px; }

/* Log Entry */
.log-entry {
  display: flex;
  align-items: baseline;
  gap: 10px;
  padding: 7px 16px;
  animation: entry-in 0.25s ease forwards;
}

@keyframes entry-in {
  from { opacity: 0; transform: translateX(-6px); }
  to   { opacity: 1; transform: translateX(0); }
}

.entry-text {
  font-size: 13px;
  color: rgba(0, 255, 100, 0.85);
  word-break: break-all;
  letter-spacing: 0.3px;
  white-space: pre-wrap;
}

/* Empty State */
.empty-state {
  text-align: center;
  padding: 28px 0;
  color: rgba(0, 255, 100, 0.18);
  font-family: 'VT323', monospace;
  font-size: 18px;
  letter-spacing: 2px;
  text-transform: uppercase;
}

/* Resize Handle */
.resize-handle {
  position: absolute;
  bottom: 0;
  right: 0;
  width: 14px;
  height: 14px;
  cursor: se-resize;
  opacity: 0.3;
}

.resize-handle::after {
  content: '';
  position: absolute;
  bottom: 4px;
  right: 4px;
  width: 6px;
  height: 6px;
  border-right: 1px solid #00ff64;
  border-bottom: 1px solid #00ff64;
}
</style>
