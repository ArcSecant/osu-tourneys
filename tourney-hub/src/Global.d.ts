declare global {
  export interface MapInfo {
    key: number
    mod?: string
    modNum: number
    cover?: string
    mapName: string
    sr: number
    bpm: number
    length: string
    cs: number
    ar: number
    od: number
    id: number
    setId?: number
  }

  export interface Mappool {
    [key: string]: Array<MapInfo>
  }

  export interface ServerResponse {
    error: string
  }
}

export {}
