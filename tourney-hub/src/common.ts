import { useState, useEffect } from "react"

export const modpool = ["NM", "HD", "HR", "DT", "FM", "EZ", "HT", "FL", "TB"]

export function convertToDataSource(mappool: Mappool): Array<MapInfo> {
  return Object.entries(mappool)
    .sort((a, b) => modpool.indexOf(a[0]) - modpool.indexOf(b[0]))
    .map((maps) => {
      let newMaps = maps[1]
      newMaps.forEach((m) => {
        m.mod = `${maps[0]}${m.modNum}`
      })
      return newMaps
    })
    .flat()
}

interface WindowSize {
  width: number
  height: number
}

export function useWindowSize(): WindowSize {
  const [windowSize, setWindowSize] = useState({
    width: 0,

    height: 0,
  })

  useEffect(() => {
    function handleResize() {
      setWindowSize({
        width: window.innerWidth,

        height: window.innerHeight,
      })
    }

    window.addEventListener("resize", handleResize)

    handleResize()

    return () => window.removeEventListener("resize", handleResize)
  }, [])

  return windowSize
}
