import React, { useState } from "react"
import { Table, Form, Input, Button, Select } from "antd"
import { FormInstance } from "antd/lib/form"
import "antd/dist/antd.css"

const { Option } = Select
const axios = require("axios").default

const modpool = ["NM", "HD", "HR", "DT", "FM", "EZ", "HT", "FL", "TB"]

const stages = [
  "Groups",
  "Qualifers",
  "RO64",
  "RO32",
  "RO16",
  "QF",
  "SF",
  "F",
  "GF",
]

function round(n: number): number {
  return Math.round((n + Number.EPSILON) * 100) / 100
}

export const Mappool: React.FC = () => {
  const [maps, setMaps] = useState<Mappool>({})

  let formRefMap = React.createRef<FormInstance>()
  let formRefPool = React.createRef<FormInstance>()

  let columns = [
    { title: "Mod", dataIndex: "mod", key: "mod" },
    { title: "Map", dataIndex: "mapName", key: "mapName" },
    { title: "SR", dataIndex: "sr", key: "sr" },
    { title: "BPM", dataIndex: "bpm", key: "bpm" },
    { title: "Length", dataIndex: "length", key: "length" },
    { title: "CS", dataIndex: "cs", key: "cs" },
    { title: "AR", dataIndex: "ar", key: "ar" },
    { title: "OD", dataIndex: "od", key: "od" },
    {
      title: "ID",
      dataIndex: "id",
      key: "id",
      render: (id: string) => (
        <a href={`https://osu.ppy.sh/b/${id}`} target="_blank">
          {id}
        </a>
      ),
    },
    {
      title: "Action",
      dataIndex: "",
      key: "x",
      render: (_text: any, record: any) => (
        <a
          onClick={(e) => {
            onDelete(record.key, e)
          }}
        >
          Delete
        </a>
      ),
    },
  ]

  let onDelete = (
    key: number,
    e: React.MouseEvent<HTMLSpanElement, MouseEvent>
  ) => {
    e.preventDefault()
    let newMappool: Mappool = {}
    Object.keys(maps).forEach((map) => {
      newMappool[map] = maps[map].filter((item: any) => item.key !== key)
    })
    setMaps(newMappool)
  }

  let onAdd = () => {
    let mapId: string = formRefMap.current!.getFieldValue("mapId")
    let mod: string = formRefMap.current!.getFieldValue("mod")
    let modNum: string = formRefMap.current!.getFieldValue("num")
    let modFlag = 0
    switch (mod) {
      case "EZ": {
        modFlag = 2
        break
      }
      case "HR": {
        modFlag = 16
        break
      }
      case "DT": {
        modFlag = 64
        break
      }
      case "HT": {
        modFlag = 256
        break
      }
      default: {
        modFlag = 0
      }
    }
    fetch(`/api/map_info/${mapId}?mods=${modFlag}`)
      .then((response) => response.json())
      .then((response) => {
        let map = {
          key: Date.now(),
          modNum: parseInt(modNum),
          mapName: `${response.beatmapset.artist} - ${response.beatmapset.mapName}[${response.diffName}]`,
          sr: round(response.starRating),
          bpm: round(response.bpm),
          length: new Date(1000 * response.drainTime)
            .toISOString()
            .substr(14, 5),
          cs: round(response.cs),
          ar: round(response.ar),
          od: round(response.od),
          id: response.id,
        }
        let newMaps = [...(maps?.[mod] ?? []), map].sort(
          (a, b) => a.modNum - b.modNum
        )
        setMaps({ ...maps, ...{ [mod]: newMaps } })
      })
  }

  let savePool = () => {
    let poolName: string = formRefPool.current!.getFieldValue("poolName")
    let poolStage: string = formRefPool.current!.getFieldValue("poolStage")
    axios
      .post("/api/save_pool", {
        poolName: poolName,
        poolStage: poolStage,
        maps: maps,
        timestamp: new Date().toISOString(),
      })
      .then((response: any) => {
        console.log(response)
      })
      .catch((err: any) => {
        console.log(err)
      })
  }

  let convertToDataSource = (mappool: Mappool) => {
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

  return (
    <div>
      <Form layout={"inline"} ref={formRefMap}>
        <Form.Item name="mapId" label="Map ID">
          <Input placeholder="Beatmap link or ID" allowClear />
        </Form.Item>
        <Form.Item name="mod" label="Mod" style={{ width: "150px" }}>
          <Select placeholder="Mod" allowClear>
            {modpool.map((mod) => {
              return <Option value={mod}>{mod}</Option>
            })}
          </Select>
        </Form.Item>
        <Form.Item name="num" style={{ width: "60px" }}>
          <Input placeholder="#" allowClear />
        </Form.Item>
        {/* <Form.Item
            noStyle
            shouldUpdate={(prevValues, currentValues) =>
              prevValues.mod !== currentValues.mod
            }
          >
            {({ getFieldValue }) =>
              getFieldValue("mod") === "Custom" ? (
                <Form.Item name="customMod" style={{ width: "50px" }}>
                  <Input />
                </Form.Item>
              ) : null
            }
          </Form.Item> */}
        <Form.Item>
          <Button type="primary" onClick={onAdd}>
            Add
          </Button>
        </Form.Item>
        <Form.Item>
          <Button type="link" href="/api/login">
            Login
          </Button>
        </Form.Item>
      </Form>
      <br />
      <Form layout={"inline"} ref={formRefPool}>
        <Form.Item name="poolName" style={{ width: "150px" }}>
          <Input placeholder="Name" allowClear />
        </Form.Item>
        <Form.Item name="poolStage" label="Stage" style={{ width: "150px" }}>
          <Select placeholder="Stage" allowClear>
            {stages.map((stage) => {
              return <Option value={stage}>{stage}</Option>
            })}
          </Select>
        </Form.Item>
        <Form.Item>
          <Button type="link" onClick={savePool}>
            Save
          </Button>
        </Form.Item>
      </Form>
      <br />
      <Table
        columns={columns}
        dataSource={convertToDataSource(maps)}
        pagination={false}
      />
    </div>
  )
}
