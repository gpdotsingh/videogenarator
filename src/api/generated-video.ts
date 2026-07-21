import { backendCall } from './backend'
import type { GalleryItem } from '../stores/createStore'

export const GENERATED_VIDEO_DIRECTORY =
  '/Users/gauravsingh/study/video/newproject/generatedvideo'

export interface GeneratedVideoArchiveResult {
  path: string
  filename: string
}

/**
 * Copies a completed local ComfyUI video into the project's stable output
 * directory. Cloud results have no local ComfyUI filename and are skipped.
 */
export async function archiveGeneratedVideo(
  item: Pick<GalleryItem, 'type' | 'filename' | 'subfolder'>,
): Promise<GeneratedVideoArchiveResult | null> {
  if (item.type !== 'video' || !item.filename) return null
  return backendCall<GeneratedVideoArchiveResult>('archive_generated_video', {
    filename: item.filename,
    subfolder: item.subfolder || '',
  })
}
