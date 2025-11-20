  <template>
    <div class="toolbar">

      <input v-model.trim="q" placeholder="搜索（玩家名/地图/备注）" class="input" />

      <select v-model="race" class="select">
        <option value="">全部种族</option>
        <option value="Zerg">Zerg</option>
        <option value="Terran">Terran</option>
        <option value="Protoss">Protoss</option>
      </select>

      <select v-model="result" class="select">
        <option value="">全部结果</option>
        <option value="胜">胜利</option>
        <option value="负">失败</option>
      </select>
      <select v-model="sortKey" class="select">
        <option value="date">按日期</option>
        <option value="title">按玩家名</option>
        <option value="map">按地图</option>
        <option value="race">按种族</option>
        <option value="result">按结果</option>
      </select>

      <button class="btn" @click="sortAsc = !sortAsc">
        {{ sortAsc ? '升序' : '降序' }}
      </button>
    </div>

    <div class="content">
    <div
      v-for="file in viewList"
      :key="file.path"
      class="card"
    >
      <div class="card-header">
        <b class="card-title">
          {{ file.title }}
          <span class="muted"> ({{ file.race }})</span>
          -
          <span>{{ file.date }}</span>
        </b>

        <b :class="{ win: file.result === '胜', loss: file.result !== '胜' }">
          {{ file.result === '胜' ? '胜利' : '失败' }}
        </b>
      </div>

      <div class="card-content">
        <div class="overview-grid">
          <div class="kv">
            <span>地图</span>
            <div class="ovt">{{ file.map }}</div>
          </div>

          <div class="kv">
            <span>对手</span>
            <div class="ovt">{{ file.versusWith }}</div>
          </div>

          <div class="kv">
            <span>时长</span>
            <div class="ovt">{{ file.durationText }}</div>
          </div>
        </div>

        <p class="card-desc">分析自Replay：{{ file.replayName }}</p>
        <p class="card-desc">
          备注：{{ file.note || '暂无备注' }}
        </p>
      </div>

      <div class="card-font">
        <div class="card-actions">
          <button class="btn-primary btn-success" @click="openExportDir_player(file.path)">
            打开分析结果(.xlsx)
          </button>
          <button class="btn-primary" @click="openFolder(file)">
            打开所在目录
          </button>
          <button class="btn-primary  btn-danger" @click="deleteRecord(file)">
            删除记录
          </button>

          <button class="btn-primary btn-ghost" @click="addNote(file)">
            添加备注
          </button>
        </div>
      </div>
</div>



    </div>
  </template>

  <script setup lang="ts">
  // import card from '../../components/widget/Data_card.vue';
  import { readDir, createDir, exists, readTextFile, writeTextFile  } from '@tauri-apps/plugin-fs'
  import { join,appDataDir,  resolveResource, BaseDirectory, dirname} from '@tauri-apps/api/path'
  import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
  import { openPath } from '@tauri-apps/plugin-opener';
  async function openFolder(file: any) {
  const dir = await dirname(file.path)

    try {
      await openPath(dir)
      console.log("✅ 已打开目录:", dir)
    } catch (e) {
      console.error("❌ 打开目录失败:", e)
    }
  }

  async function openExportDir_player(dir: string) {
    console.log("打开:",dir)

    try {
      await openPath(dir)
      console.log("✅ 已打开导出目录:", dir)
    } catch (e) {
      console.error("❌ 打开目录失败:", e)
    }
  }
  const INDEX_FILE = 'replay_index.json'

  // 响应式状态
  const q = ref('')             // 关键字
  const race = ref('')          // Zerg / Terran / Protoss / ''
  const result = ref('')        // 胜利 / 失败 / ''
  const dateStart = ref<string>('') // 'YYYY-MM-DD'
  const dateEnd   = ref<string>('') // 'YYYY-MM-DD'

  const sortKey = ref<'date'|'title'|'mapName'|'race'|'result'>('date')
  const sortAsc = ref(false)


  const toDate = (s: string) => (s ? new Date(s) : null)
  const contains = (hay: string, needle: string) => hay.toLowerCase().includes(needle.toLowerCase())

  // 过滤 + 排序
  const viewList = computed(() => {
    let arr = xlsxFiles.value

    // 过滤
    if (q.value) {
      arr = arr.filter(it =>
        contains(it.title, q.value) ||
        contains(it.map, q.value) ||
        contains(it.race, q.value) ||
        contains(it.result, q.value) ||
        contains(it.date, q.value) ||
        contains(it.note, q.value)
      )
    }
    if (race.value) {
      arr = arr.filter(it => it.race === race.value)
    }
    if (result.value) {
      arr = arr.filter(it => it.result === result.value)
    }
    if (dateStart.value || dateEnd.value) {
      const s = toDate(dateStart.value)
      const e = toDate(dateEnd.value)
      arr = arr.filter(it => {
        const d = toDate(it.date)
        if (!d) return false
        if (s && d < s) return false
        if (e && d > e) return false
        return true
      })
    }

    // 排序
    const withIdx = arr.map((it, idx) => ({ it, idx }))
    withIdx.sort((a, b) => {
      let cmp = 0
      if (sortKey.value === 'date') {
        const da = toDate(a.it.date)?.getTime() ?? 0
        const db = toDate(b.it.date)?.getTime() ?? 0
        cmp = da - db
      } else {
        const ka = String(a.it[sortKey.value] ?? '')
        const kb = String(b.it[sortKey.value] ?? '')
        cmp = ka.localeCompare(kb)
      }
      if (!sortAsc.value) cmp = -cmp
      return cmp || (a.idx - b.idx)
    })
    return withIdx.map(x => x.it)
  })

  const baseName = (full: string) => {
    // 处理 Windows 路径（\）和 / 混用的情况
    return full.replace(/[/\\]+$/, '').split(/[/\\]/).pop() || full
  }
  const formatDuration = (sec: number): string => {
    if (!sec || sec < 0) return ''
    const m = Math.floor(sec / 60)
    const s = sec % 60
    const mm = String(m)           // 分钟可以不补零，看你喜好
    const ss = String(s).padStart(2, '0')
    return `${mm}:${ss}`
  }

type XlsxInfo = {
  name: string
  path: string
  title: string       // 当前视角玩家名
  map: string
  race: string
  date: string
  result: string      // '胜' / '负'
  versusWith: string
  durationSec: number     // 总时长（秒）
  durationText: string    // 展示用文本：09:27 之类
  replayName: string
  note: string
}

  const xlsxFiles = ref<XlsxInfo[]>([])

async function loadFromIndexJson(): Promise<XlsxInfo[]> {
  const base = await appDataDir()
  const folder = await join(base, 'index')
  const indexPath = await join(folder, INDEX_FILE)

  if (!(await exists(indexPath))) {
    console.warn('索引文件不存在，返回空：', indexPath)
    return []
  }

  let raw: any
  try {
    const text = await readTextFile(indexPath)
    raw = JSON.parse(text)
  } catch (err) {
    console.error('读取或解析索引失败：', err)
    return []
  }

  const arr: any[] = Array.isArray(raw) ? raw : [raw]
  const out: XlsxInfo[] = []

  for (const match of arr) {
    if (!match || typeof match !== 'object') continue

    const mapName = String(match.map_name ?? '')
    const date = match.endTime


    const durationSec = Number(match.duration ?? 0) || 0
    const durationText = formatDuration(durationSec)
    const replayName = String(match.output ?? '')

    const playersInfo = match.playersInfo || {}
    const playerNames: string[] = Object.keys(playersInfo)
    for (const playerName of playerNames) {
      const p = playersInfo[playerName]
      const note = p.note
      const versusWith = String(p.versus ?? '')
      if (!p || typeof p !== 'object') continue

      const outputPath = String(p.outputPath ?? '')
      if (!outputPath) continue

      out.push({
        path: outputPath,

        name: baseName(outputPath),
        title: playerName,
        map: mapName,
        race: String(p.race ?? ''),
        date,
        result: p.result ? '胜' : '负',
        durationSec,
        versusWith,
        durationText,
        replayName: baseName(replayName),
        note,
      })
    }
  }

  return out
}

async function deleteRecord(file: XlsxInfo) {
  const base = await appDataDir()
  const folder = await join(base, 'index')
  const indexPath = await join(folder, INDEX_FILE)

  let raw: any
  try {
    const text = await readTextFile(indexPath)
    raw = JSON.parse(text)
  } catch (err) {
    console.error('读取索引失败，无法删除记录:', err)
    return
  }

  const arr: any[] = Array.isArray(raw) ? raw : [raw]
  const newArr: any[] = []

  for (const match of arr) {
    if (!match || typeof match !== 'object') continue

    const playersInfo = match.playersInfo || {}
    let changed = false

    for (const playerName of Object.keys(playersInfo)) {
      const p = playersInfo[playerName]
      if (!p || typeof p !== 'object') continue
      const outputPath = String(p.outputPath ?? '')
      if (outputPath === file.path) {
        delete playersInfo[playerName]
        changed = true
      }
    }

    const remainingPlayers = Object.keys(playersInfo)
    if (remainingPlayers.length > 0) {
      newArr.push(match)
    } else if (!changed) {
      // 没有改动的 match 也要保留
      newArr.push(match)
    } // 如果 changed 且玩家为 0，就丢弃整个 match
  }

  try {
    await writeTextFile(indexPath, JSON.stringify(newArr, null, 2))
    console.log('✅ 索引已更新')
  } catch (e) {
    console.error('❌ 写入索引失败:', e)
  }

  // 删除对应的xlsx文件：TODO

  // 刷新列表
  await refresh()
}

async function addNote(file: XlsxInfo) {
  const old = file.note || ''
  const text = window.prompt('请输入备注：', old)
  if (text === null) {
    return
  }

  file.note = text

  const base = await appDataDir()
  const folder = await join(base, 'index')
  const indexPath = await join(folder, INDEX_FILE)

  let raw: any
  try {
    const t = await readTextFile(indexPath)
    raw = JSON.parse(t)
  } catch (e) {
    console.error('读取索引失败，无法保存备注:', e)
    return
  }

  const arr: any[] = Array.isArray(raw) ? raw : [raw]

  // 通过 outputPath 匹配对应的条目
  for (const match of arr) {
    if (!match || typeof match !== 'object') continue
    const playersInfo = match.playersInfo || {}
    for (const playerName of Object.keys(playersInfo)) {
      const p = playersInfo[playerName]
      if (!p || typeof p !== 'object') continue

      const outputPath = String(p.outputPath ?? '')
      if (outputPath === file.path) {
        // 找到对应的玩家，写入 note
        p.note = text
      }
    }
  }

  try {
    await writeTextFile(indexPath, JSON.stringify(arr, null, 2))
  } catch (e) {
  }
  await refresh()
}


  let timer: number | null = null

  async function refresh() {
    xlsxFiles.value = await loadFromIndexJson()
  }

  onMounted(async () => {
    await refresh()
    timer = window.setInterval(refresh, 3000)
  })

  onBeforeUnmount(() => {
    if (timer !== null) clearInterval(timer)
  })

  </script>

  <style scoped>
  .toolbar { 
    position: fixed;
    width: 100%;
    height: var(--nav-height);
    display: flex;
    gap: 16px;
    background-color: #0f1012;
    padding: 8px 12px;
    user-select: none;
    border-bottom: 1px solid #2d2d2d;
    z-index: 10;
  }

  .select, .btn {
      padding: .4rem .6rem;
      
      border-radius: .4rem;
      border: 1px solid #333;
      background:#1f1f1f;
      color:#eee; }

  .input{
      margin: 2px 5px;
      padding: .4rem .6rem;
      border-radius: .4rem;
      border: 1px solid #333;
      background:#1f1f1f;
      color:#eee; 
  }

  .btn { cursor: pointer; }

  .content{
    flex: 1;
    height: calc(100vh - 50px);
    margin-top: 50px;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 5px 5px;

    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(600px, 1fr));
    gap: 12px; /* 卡片间距 */
     align-items: start;
  }

  #result {
  background-color: #10b981;
}
#rename {
  background-color: #ffffff;
  color: black;
}


.card {
  background: rgba(192, 114, 114, 0.05);
  backdrop-filter: blur(8px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  overflow: hidden;
  transition: transform 0.2s ease, box-shadow 0.1s ease;
  margin: 4px 12px;
  height:max-content;
  padding: 12px;
}

.btn-primary {
  background-color: #3b82f6;
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.btn-primary:hover {
  background-color: #2563eb;
}


.card:hover {
   /* transform: translateY(-4px); */
  box-shadow: 0 0 10px 2px rgba(233, 233, 233, 0.2);
  background-color: rgba(58, 56, 56, 0.61);
}
.card-header {
  display: flex; 
  align-items: center; 
  justify-content: space-between;
  gap: 10px;
  margin-bottom: 8px; 
}

.card-content {
  padding: 0px;
}
.overview-grid { display: grid; grid-template-columns: repeat(3, minmax(0,1fr)); gap: 10px; }

.card-title {
  font-size: 16px;
  font-weight: 600;
  margin: 2px 2px;
  color: white;
}

.card-desc {
  font-size: 14px;
  color: #b3b3b3;
  margin: 5px 5px;
}

.card-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}
.card-count {
  font-size: 12px;
  color: #9ca3af;
}


.kv { background: #111418; border: 1px solid var(--border); border-radius: 12px; padding: 12px; display: flex; flex-direction: column; gap: 6px; }
.kv span { color: var(--muted); font-size: 12px; }
.kv b { font-size: 16px; }
.kv b.win { color: var(--success); }

b.win {
  color: var(--success);
  padding: 0px 10px;
}
b.loss {
  color: #f87171;
  padding: 0px 10px;
}
.btn-success { background-color: #10b981; }
.btn-success:hover { background-color: #059669; }


.btn-ghost {
  background-color: var(--btn-ghost-bg);
  color: var(--text-main);
}

.btn-ghost:hover {
  background-color: var(--btn-ghost-bg);
  color: var(--text-main);
}


.btn-danger {
  background: #f03838;
}

.btn-danger:hover {
  background: #bb1212;
}
  </style>


