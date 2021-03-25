import React, { useState } from "react"
import { Table, Form, Input, Button, Select } from "antd"
import { FormInstance } from "antd/lib/form"
import "antd/dist/antd.css"

const { Option } = Select

const modpool = ["NM", "HD", "HR", "DT", "FM", "EZ", "HT", "FL", "TB"]

const columns = [
  { title: "Mod", dataIndex: "mod", key: "mod" },
  { title: "Map", dataIndex: "map", key: "map" },
  { title: "SR", dataIndex: "sr", key: "sr" },
  { title: "BPM", dataIndex: "bpm", key: "bpm" },
  { title: "Length", dataIndex: "length", key: "length" },
  { title: "CS", dataIndex: "cs", key: "cs" },
  { title: "AR", dataIndex: "ar", key: "ar" },
  { title: "OD", dataIndex: "od", key: "od" },
  { title: "ID", dataIndex: "id", key: "id" },
]

interface MapInfo {
  mod: string
  map: string
  sr: number
  bpm: number
  length: string
  cs: number
  ar: number
  od: number
  id: number
}

interface IProps {}
interface IState {
  maps: Array<MapInfo>
}

function round(n: number): number {
  return Math.round((n + Number.EPSILON) * 100) / 100
}

class Mappool extends React.Component<IProps, IState> {
  constructor(props: IProps) {
    super(props)

    this.state = {
      maps: [],
    }
  }
  formRef = React.createRef<FormInstance>()

  fetchMapInfo = (mapId: string, mod: string) => {
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
          mod: mod ?? "NM",
          map: `${response.beatmapset.artist} - ${response.beatmapset.mapName}[${response.diffName}]`,
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
        this.setState({ maps: [...this.state.maps, map] })
      })
  }

  onAdd = () => {
    let mapId: string = this.formRef.current!.getFieldValue("map")
    let mod: string = this.formRef.current!.getFieldValue("mod")
    this.fetchMapInfo(mapId, mod)
  }

  render() {
    return (
      <div>
        <Form layout={"inline"} ref={this.formRef}>
          <Form.Item name="map" label="Map ID">
            <Input placeholder="Beatmap link or ID" allowClear />
          </Form.Item>
          <Form.Item name="mod" label="Mod" style={{ width: "150px" }}>
            <Select placeholder="Mod" allowClear>
              {modpool.map((mod) => {
                return <Option value={mod}>{mod}</Option>
              })}
            </Select>
          </Form.Item>
          <Form.Item
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
          </Form.Item>
          <Form.Item>
            <Button type="primary" onClick={this.onAdd}>
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
        <Table
          columns={columns}
          dataSource={this.state.maps}
          pagination={false}
        />
      </div>
    )
  }
}

export { Mappool }
