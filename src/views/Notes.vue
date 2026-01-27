<script setup lang="ts">
import { ref, onMounted, computed, onBeforeUnmount } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useEditor, EditorContent } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import TaskList from '@tiptap/extension-task-list';
import TaskItem from '@tiptap/extension-task-item';

interface Note {
  id: string;
  title: string;
  content: string;
  createdAt: string;
  updatedAt: string;
}

const notes = ref<Note[]>([]);
const currentNoteId = ref<string | null>(null);
const editTitle = ref('');
const searchQuery = ref('');
const dateFilter = ref<string>('');  // 日期筛选，格式: YYYY-MM-DD

const editor = useEditor({
  content: '',
  extensions: [
    StarterKit,
    TaskList,
    TaskItem.configure({
      nested: true,
    }),
  ],
  onUpdate: ({ editor }) => {
    // 自动保存内容
    if (currentNoteId.value) {
      const note = notes.value.find(n => n.id === currentNoteId.value);
      if (note) {
        note.content = editor.getHTML();
        note.updatedAt = new Date().toISOString();
        saveNotes();
      }
    }
  }
});

const currentNote = computed(() => {
  return notes.value.find(n => n.id === currentNoteId.value) || null;
});

const filteredNotes = computed(() => {
  let result = notes.value;
  
  // 日期筛选
  if (dateFilter.value) {
    result = result.filter(n => {
      const noteDate = new Date(n.createdAt).toISOString().split('T')[0];
      return noteDate === dateFilter.value;
    });
  }
  
  // 搜索筛选
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase();
    result = result.filter(n => 
      n.title.toLowerCase().includes(query) || 
      n.content.toLowerCase().includes(query)
    );
  }
  
  return result;
});

// 清除日期筛选
function clearDateFilter() {
  dateFilter.value = '';
}

async function loadNotes() {
  try {
    const data = await invoke<string>('load_notes');
    if (data) {
      notes.value = JSON.parse(data);
      if (notes.value.length > 0 && !currentNoteId.value) {
        selectNote(notes.value[0].id);
      }
    }
  } catch (e) {
    console.warn('加载笔记失败:', e);
  }
}

async function saveNotes() {
  try {
    await invoke('save_notes', { 
      notes: JSON.stringify(notes.value) 
    });
  } catch (e) {
    console.error('保存笔记失败:', e);
  }
}

function createNote() {
  const note: Note = {
    id: Date.now().toString(),
    title: '新笔记',
    content: '',
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString()
  };
  notes.value.unshift(note);
  selectNote(note.id);
  saveNotes();
}

function selectNote(id: string) {
  currentNoteId.value = id;
  const note = notes.value.find(n => n.id === id);
  if (note) {
    editTitle.value = note.title;
    editor.value?.commands.setContent(note.content || '');
  }
}

function saveTitle() {
  if (!currentNoteId.value) return;
  const note = notes.value.find(n => n.id === currentNoteId.value);
  if (note) {
    note.title = editTitle.value || '无标题';
    note.updatedAt = new Date().toISOString();
    saveNotes();
  }
}

function deleteNote(id: string) {
  const index = notes.value.findIndex(n => n.id === id);
  if (index > -1) {
    notes.value.splice(index, 1);
    if (currentNoteId.value === id) {
      currentNoteId.value = notes.value.length > 0 ? notes.value[0].id : null;
      if (currentNoteId.value) {
        selectNote(currentNoteId.value);
      }
    }
    saveNotes();
  }
}

function formatDate(dateStr: string) {
  const date = new Date(dateStr);
  return date.toLocaleDateString('zh-CN', { 
    month: 'short', 
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
}

onMounted(() => {
  loadNotes();
});

onBeforeUnmount(() => {
  editor.value?.destroy();
});
</script>

<template>
  <div class="notes-page">
    <!-- 笔记列表侧边栏 -->
    <div class="notes-sidebar">
      <div class="sidebar-header">
        <h3>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48" width="20" height="20" style="vertical-align: middle; margin-right: 6px;">
            <path fill="#ffd54f" d="M40,45H8c-1.1,0-2-0.9-2-2V5c0-1.1,0.9-2,2-2h24l10,10v30C42,44.1,41.1,45,40,45z"></path>
            <path fill="#ffecb3" d="M38.5,14H29V4.5L38.5,14z"></path>
            <path fill="#9e9e9e" d="M16 21H33V23H16zM16 25H33V27H16zM16 29H33V31H16zM16 33H25V35H16z"></path>
          </svg>
          记事本
        </h3>
        <button class="new-note-btn" @click="createNote">+ 新建</button>
      </div>
      <div class="filter-box">
        <input 
          v-model="searchQuery" 
          type="text" 
          placeholder="搜索笔记..." 
          class="search-input"
        />
        <div class="date-filter">
          <input 
            type="date" 
            v-model="dateFilter" 
            class="date-input"
          />
          <button v-if="dateFilter" class="clear-filter-btn" @click="clearDateFilter" title="清除筛选">×</button>
        </div>
      </div>
      <div class="notes-list">
        <div 
          v-for="note in filteredNotes" 
          :key="note.id" 
          :class="['note-item', { active: note.id === currentNoteId }]"
          @click="selectNote(note.id)"
        >
          <div class="note-info">
            <span class="note-title">{{ note.title }}</span>
            <span class="note-date">{{ formatDate(note.updatedAt) }}</span>
          </div>
          <button class="delete-btn" @click.stop="deleteNote(note.id)">×</button>
        </div>
        <div v-if="filteredNotes.length === 0" class="empty-list">
          暂无笔记
        </div>
      </div>
    </div>

    <!-- 笔记内容区 -->
    <div class="notes-main">
      <template v-if="currentNote">
        <div class="note-header">
          <input 
            v-model="editTitle" 
            class="title-input"
            placeholder="输入标题..."
            @blur="saveTitle"
            @keyup.enter="($event.target as HTMLInputElement).blur()"
          />
        </div>
        <!-- 工具栏 -->
        <div class="editor-toolbar" v-if="editor">
          <button 
            @click="editor.chain().focus().toggleBold().run()"
            :class="{ active: editor.isActive('bold') }"
            title="粗体"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path>
              <path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path>
            </svg>
          </button>
          <button 
            @click="editor.chain().focus().toggleItalic().run()"
            :class="{ active: editor.isActive('italic') }"
            title="斜体"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="19" y1="4" x2="10" y2="4"></line>
              <line x1="14" y1="20" x2="5" y2="20"></line>
              <line x1="15" y1="4" x2="9" y2="20"></line>
            </svg>
          </button>
          <button 
            @click="editor.chain().focus().toggleStrike().run()"
            :class="{ active: editor.isActive('strike') }"
            title="删除线"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="5" y1="12" x2="19" y2="12"></line>
              <path d="M16 6C16 6 14 4 12 4C10 4 8 5.5 8 8C8 10.5 10 11 12 11"></path>
              <path d="M8 18C8 18 10 20 12 20C14 20 16 18.5 16 16C16 13.5 14 13 12 13"></path>
            </svg>
          </button>
          <span class="toolbar-divider"></span>
          <button 
            @click="editor.chain().focus().toggleHeading({ level: 1 }).run()"
            :class="{ active: editor.isActive('heading', { level: 1 }) }"
            title="标题1"
          >H1</button>
          <button 
            @click="editor.chain().focus().toggleHeading({ level: 2 }).run()"
            :class="{ active: editor.isActive('heading', { level: 2 }) }"
            title="标题2"
          >H2</button>
          <button 
            @click="editor.chain().focus().toggleHeading({ level: 3 }).run()"
            :class="{ active: editor.isActive('heading', { level: 3 }) }"
            title="标题3"
          >H3</button>
          <span class="toolbar-divider"></span>
          <button 
            @click="editor.chain().focus().toggleBulletList().run()"
            :class="{ active: editor.isActive('bulletList') }"
            title="无序列表"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="9" y1="6" x2="20" y2="6"></line>
              <line x1="9" y1="12" x2="20" y2="12"></line>
              <line x1="9" y1="18" x2="20" y2="18"></line>
              <circle cx="4" cy="6" r="2" fill="currentColor"></circle>
              <circle cx="4" cy="12" r="2" fill="currentColor"></circle>
              <circle cx="4" cy="18" r="2" fill="currentColor"></circle>
            </svg>
          </button>
          <button 
            @click="editor.chain().focus().toggleOrderedList().run()"
            :class="{ active: editor.isActive('orderedList') }"
            title="有序列表"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="10" y1="6" x2="21" y2="6"></line>
              <line x1="10" y1="12" x2="21" y2="12"></line>
              <line x1="10" y1="18" x2="21" y2="18"></line>
              <text x="2" y="8" font-size="8" fill="currentColor">1</text>
              <text x="2" y="14" font-size="8" fill="currentColor">2</text>
              <text x="2" y="20" font-size="8" fill="currentColor">3</text>
            </svg>
          </button>
          <button 
            @click="editor.chain().focus().toggleTaskList().run()"
            :class="{ active: editor.isActive('taskList') }"
            title="待办事项"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="5" width="6" height="6" rx="1"></rect>
              <path d="M5 11l2-2 2 2"></path>
              <line x1="12" y1="8" x2="21" y2="8"></line>
              <rect x="3" y="14" width="6" height="6" rx="1"></rect>
              <line x1="12" y1="17" x2="21" y2="17"></line>
            </svg>
          </button>
          <button 
            @click="editor.chain().focus().toggleBlockquote().run()"
            :class="{ active: editor.isActive('blockquote') }"
            title="引用"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V21z"></path>
              <path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v4z"></path>
            </svg>
          </button>
          <button 
            @click="editor.chain().focus().toggleCodeBlock().run()"
            :class="{ active: editor.isActive('codeBlock') }"
            title="代码块"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="16 18 22 12 16 6"></polyline>
              <polyline points="8 6 2 12 8 18"></polyline>
            </svg>
          </button>
        </div>
        <!-- 编辑器内容 -->
        <div class="note-content">
          <EditorContent :editor="editor" class="tiptap-editor" />
        </div>
      </template>
      <div v-else class="empty-state">
        <svg class="empty-icon-svg" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48" width="64" height="64">
          <path fill="#ffd54f" d="M40,45H8c-1.1,0-2-0.9-2-2V5c0-1.1,0.9-2,2-2h24l10,10v30C42,44.1,41.1,45,40,45z"></path>
          <path fill="#ffecb3" d="M38.5,14H29V4.5L38.5,14z"></path>
          <path fill="#9e9e9e" d="M16 21H33V23H16zM16 25H33V27H16zM16 29H33V31H16zM16 33H25V35H16z"></path>
        </svg>
        <p>选择或创建一个笔记</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.notes-page {
  display: flex;
  height: calc(100vh - 60px);
  background: #f8fafc;
}

.notes-sidebar {
  width: 280px;
  min-width: 280px;
  background: #fff;
  border-right: 1px solid #e5e7eb;
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: 16px;
  border-bottom: 1px solid #e5e7eb;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.sidebar-header h3 {
  margin: 0;
  font-size: 1rem;
  color: #374151;
}

.new-note-btn {
  padding: 6px 12px;
  background: #ffd54f;
  color: #5d4037;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.875rem;
  font-weight: 500;
}

.new-note-btn:hover {
  background: #ffca28;
}

.filter-box {
  padding: 12px 16px;
  border-bottom: 1px solid #e5e7eb;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.search-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #e5e7eb;
  border-radius: 6px;
  font-size: 0.875rem;
}

.search-input:focus {
  outline: none;
  border-color: #ffd54f;
}

.date-filter {
  display: flex;
  gap: 4px;
  align-items: center;
}

.date-input {
  flex: 1;
  padding: 6px 10px;
  border: 1px solid #e5e7eb;
  border-radius: 6px;
  font-size: 0.8rem;
  background: #fff;
  cursor: pointer;
}

.date-input:focus {
  outline: none;
  border-color: #ffd54f;
}

.clear-filter-btn {
  width: 24px;
  height: 24px;
  padding: 0;
  border: none;
  background: #fee2e2;
  color: #ef4444;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.clear-filter-btn:hover {
  background: #fecaca;
}

.notes-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.note-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  border-radius: 8px;
  cursor: pointer;
  margin-bottom: 4px;
}

.note-item:hover {
  background: #f3f4f6;
}

.note-item.active {
  background: #fef3c7;
}

.note-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  overflow: hidden;
}

.note-title {
  font-weight: 500;
  color: #374151;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.note-date {
  font-size: 0.75rem;
  color: #9ca3af;
}

.delete-btn {
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: #9ca3af;
  cursor: pointer;
  border-radius: 4px;
  font-size: 1rem;
  opacity: 0;
}

.note-item:hover .delete-btn {
  opacity: 1;
}

.delete-btn:hover {
  background: #fee2e2;
  color: #ef4444;
}

.empty-list {
  text-align: center;
  padding: 40px 20px;
  color: #9ca3af;
}

.notes-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 24px;
  overflow: hidden;
}

.note-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.note-header h2 {
  margin: 0;
  font-size: 1.5rem;
  color: #1f2937;
  cursor: pointer;
}

.title-input {
  flex: 1;
  font-size: 1.5rem;
  font-weight: 600;
  border: none;
  border-bottom: 2px solid #ffd54f;
  padding: 4px 0;
  outline: none;
  background: transparent;
}

.note-actions button {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.875rem;
}

.edit-btn {
  background: #f3f4f6;
  color: #374151;
}

.edit-btn:hover {
  background: #e5e7eb;
}

.save-btn {
  background: #ffd54f;
  color: #5d4037;
}

.save-btn:hover {
  background: #ffca28;
}

.note-content {
  flex: 1;
  overflow: hidden;
}

.content-input {
  width: 100%;
  height: 100%;
  padding: 16px;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  font-size: 1rem;
  line-height: 1.6;
  resize: none;
  outline: none;
}

.content-input:focus {
  border-color: #ffd54f;
}

.content-display {
  height: 100%;
  padding: 16px;
  background: #fff;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  overflow-y: auto;
  cursor: pointer;
  white-space: pre-wrap;
  line-height: 1.6;
}

.content-display .placeholder {
  color: #9ca3af;
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #9ca3af;
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 16px;
}

/* 工具栏样式 */
.editor-toolbar {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 8px 12px;
  background: #fff;
  border: 1px solid #e5e7eb;
  border-radius: 8px 8px 0 0;
  border-bottom: none;
  flex-wrap: wrap;
}

.editor-toolbar button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  padding: 0;
  border: none;
  background: transparent;
  border-radius: 4px;
  cursor: pointer;
  color: #6b7280;
  font-size: 0.75rem;
  font-weight: 600;
}

.editor-toolbar button:hover {
  background: #f3f4f6;
  color: #374151;
}

.editor-toolbar button.active {
  background: #fef3c7;
  color: #92400e;
}

.toolbar-divider {
  width: 1px;
  height: 24px;
  background: #e5e7eb;
  margin: 0 4px;
}

/* TipTap编辑器样式 */
.tiptap-editor {
  height: 100%;
  overflow-y: auto;
}

.tiptap-editor :deep(.ProseMirror) {
  height: 100%;
  min-height: 300px;
  padding: 16px;
  background: #fff;
  border: 1px solid #e5e7eb;
  border-radius: 0 0 8px 8px;
  outline: none;
  font-size: 1rem;
  line-height: 1.6;
}

.tiptap-editor :deep(.ProseMirror:focus) {
  border-color: #ffd54f;
}

.tiptap-editor :deep(.ProseMirror p) {
  margin: 0 0 0.75em 0;
}

.tiptap-editor :deep(.ProseMirror h1) {
  font-size: 1.75rem;
  font-weight: 700;
  margin: 0 0 0.5em 0;
  color: #1f2937;
}

.tiptap-editor :deep(.ProseMirror h2) {
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0 0 0.5em 0;
  color: #374151;
}

.tiptap-editor :deep(.ProseMirror h3) {
  font-size: 1.25rem;
  font-weight: 600;
  margin: 0 0 0.5em 0;
  color: #4b5563;
}

.tiptap-editor :deep(.ProseMirror ul),
.tiptap-editor :deep(.ProseMirror ol) {
  padding-left: 1.5em;
  margin: 0 0 0.75em 0;
}

.tiptap-editor :deep(.ProseMirror li) {
  margin-bottom: 0.25em;
}

.tiptap-editor :deep(.ProseMirror blockquote) {
  border-left: 3px solid #ffd54f;
  padding-left: 1em;
  margin: 0 0 0.75em 0;
  color: #6b7280;
  font-style: italic;
}

.tiptap-editor :deep(.ProseMirror pre) {
  background: #1f2937;
  color: #e5e7eb;
  padding: 12px 16px;
  border-radius: 6px;
  font-family: 'Fira Code', monospace;
  font-size: 0.875rem;
  overflow-x: auto;
  margin: 0 0 0.75em 0;
}

.tiptap-editor :deep(.ProseMirror code) {
  background: #f3f4f6;
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Fira Code', monospace;
  font-size: 0.875rem;
}

.tiptap-editor :deep(.ProseMirror pre code) {
  background: transparent;
  padding: 0;
}

.tiptap-editor :deep(.ProseMirror strong) {
  font-weight: 600;
}

.tiptap-editor :deep(.ProseMirror em) {
  font-style: italic;
}

.tiptap-editor :deep(.ProseMirror s) {
  text-decoration: line-through;
}

/* 待办事项列表样式 */
.tiptap-editor :deep(.ProseMirror ul[data-type="taskList"]) {
  list-style: none;
  padding-left: 0;
  margin: 0 0 0.75em 0;
}

.tiptap-editor :deep(.ProseMirror ul[data-type="taskList"] li) {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  margin-bottom: 4px;
}

.tiptap-editor :deep(.ProseMirror ul[data-type="taskList"] li > label) {
  flex-shrink: 0;
  margin-top: 2px;
}

.tiptap-editor :deep(.ProseMirror ul[data-type="taskList"] li > label input[type="checkbox"]) {
  width: 16px;
  height: 16px;
  cursor: pointer;
  accent-color: #ffd54f;
}

.tiptap-editor :deep(.ProseMirror ul[data-type="taskList"] li > div) {
  flex: 1;
}

.tiptap-editor :deep(.ProseMirror ul[data-type="taskList"] li[data-checked="true"] > div) {
  text-decoration: line-through;
  color: #9ca3af;
}

@media (max-width: 768px) {
  .notes-sidebar {
    width: 200px;
    min-width: 200px;
  }
}

@media (max-width: 600px) {
  .notes-page {
    flex-direction: column;
  }
  .notes-sidebar {
    width: 100%;
    min-width: 100%;
    height: auto;
    max-height: 250px;
    border-right: none;
    border-bottom: 1px solid #e5e7eb;
  }
}
</style>
