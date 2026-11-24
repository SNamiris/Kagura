import { useState, type ReactNode } from 'react'
import { TriangleAlert } from 'lucide-react'
import type { AssetDetail } from './generated/AssetDetail'
import type { ModelFileInfo } from './generated/ModelFileInfo'
import type { MotionFileInfo } from './generated/MotionFileInfo'
import { number } from './presentation'
import './parsed-files.css'

function count(value: number | null) {
  return value === null ? '—' : number(value)
}

function Metric({ label, children, warning = false }: { label: string, children: ReactNode, warning?: boolean }) {
  return <div><dt>{label}</dt><dd className={warning ? 'parse-warning' : undefined}>{children}</dd></div>
}

function ModelReadout({ info }: { info: ModelFileInfo }) {
  return <>
    <div className="parse-caption"><strong>{info.name || '未填写模型名'}</strong>
      <span>{info.format.toUpperCase()} {info.version?.toFixed(1)}</span></div>
    <dl className="parse-counts">
      <Metric label="顶点">{count(info.vertices)}</Metric>
      <Metric label="骨骼">{count(info.bones)}</Metric>
      <Metric label="表情">{count(info.morphs)}</Metric>
    </dl>
    <dl className="parse-properties">
      <Metric label="标准骨骼">{info.standard_bones === null ? '—' : info.standard_bones ? '齐全' : '不齐全'}</Metric>
      <Metric label="物理">{info.has_physics === null ? '—' : info.has_physics ? '有' : '无'}</Metric>
      <Metric label="贴图">{count(info.textures)}</Metric>
      <Metric label="缺失贴图" warning={!!info.missing_textures}>{count(info.missing_textures)}</Metric>
    </dl>
    <details className="parse-extra"><summary>模型头信息</summary>
      <dl className="fields"><Metric label="英文名">{info.name_en || '—'}</Metric>
        <Metric label="编码">{info.encoding || '—'}</Metric>
        <Metric label="三角面">{count(info.faces)}</Metric><Metric label="材质">{count(info.materials)}</Metric>
        <Metric label="刚体">{count(info.rigid_bodies)}</Metric><Metric label="关节">{count(info.joints)}</Metric>
        {info.comment && <Metric label="注释">{info.comment}</Metric>}
        {info.comment_en && <Metric label="英文注释">{info.comment_en}</Metric>}</dl>
    </details>
  </>
}

function motionKind(info: MotionFileInfo) {
  const kinds: string[] = []
  if (info.is_camera || info.camera_keys) kinds.push('镜头')
  if (info.bone_keys) kinds.push('动作')
  if (info.morph_keys) kinds.push('表情')
  if (info.light_keys) kinds.push('光照')
  if (!kinds.length && info.shadow_keys) kinds.push('自阴影')
  if (!kinds.length && info.ik_keys) kinds.push('IK 显示')
  return kinds.join('、') || '无关键帧'
}

function MotionReadout({ info }: { info: MotionFileInfo }) {
  return <>
    <div className="parse-caption"><strong>{motionKind(info)}</strong><span>VMD</span></div>
    <dl className="parse-target"><Metric label="目标模型">{info.target_model || '未填写'}</Metric></dl>
    <dl className="parse-counts motion-counts">
      <Metric label="最大帧号">{count(info.max_frame)}</Metric>
      <Metric label="时长（30 fps）">{info.seconds === null ? '—' : <>{info.seconds.toLocaleString('zh-CN', { maximumFractionDigits: 2 })} <small>秒</small></>}</Metric>
    </dl>
    <dl className="parse-properties">
      <Metric label="骨骼关键帧">{count(info.bone_keys)}</Metric>
      <Metric label="表情关键帧">{count(info.morph_keys)}</Metric>
      <Metric label="镜头关键帧">{count(info.camera_keys)}</Metric>
      <Metric label="光照关键帧">{count(info.light_keys)}</Metric>
      <Metric label="自阴影帧">{count(info.shadow_keys)}</Metric>
      <Metric label="IK 显示帧">{count(info.ik_keys)}</Metric>
    </dl>
  </>
}

function FileReadout<T extends ModelFileInfo | MotionFileInfo>({ title, entries, preferredFile, render }: {
  title: string, entries: T[], preferredFile?: string, render: (entry: T) => ReactNode,
}) {
  const [selection, setSelection] = useState('')
  const entry = entries.find(item => item.file === selection)
    ?? entries.find(item => item.file === preferredFile) ?? entries[0]
  if (!entry) return null
  const error = entry.parse_error
  return <section className="detail-section parse-section" aria-label={title}>
    <h3>{title} <small>{number(entries.length)} 个文件</small></h3>
    {entries.length > 1 ? <select aria-label={title + '文件'} title={entry.file} value={entry.file} onChange={e => setSelection(e.target.value)}>
      {entries.map(item => <option value={item.file} key={item.file}>{item.file}</option>)}
    </select> : <p className="parse-path">{entry.file}</p>}
    {error ? <div className="parse-failure"><TriangleAlert size={16}/><div><strong>解析失败</strong>
      <p>{error.section} · 偏移 {number(error.offset)} 字节</p><p>{error.message}</p></div></div> : render(entry)}
  </section>
}

export function ParsedFiles({ detail, primary }: { detail: AssetDetail, primary?: string }) {
  return <>
    <FileReadout title="模型解析" entries={detail.model_info} preferredFile={primary} render={info => <ModelReadout info={info}/>}/>
    <FileReadout title="动作解析" entries={detail.motion_info} render={info => <MotionReadout info={info}/>}/>
  </>
}
