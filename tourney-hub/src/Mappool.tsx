import React, { useState, useEffect } from "react"
import { useParams } from "react-router-dom"
import { Table, Image, Form, Input, Button, Select } from "antd"

import { convertToDataSource, useWindowSize } from "./common"

interface IProps {
  poolName: string
}

const ViewMappool: React.FC = () => {
  const windowSize = useWindowSize()
  let [mappool, setMappool] = useState<Mappool>({})

  let { poolName } = useParams<IProps>()
  let columns = [
    { title: "Mod", dataIndex: "mod", key: "mod", width: "2.5%" },
    {
      title: "Cover",
      dataIndex: "cover",
      key: "cover",
      width: "10%",
      render: (url: string) => <Image fallback={"https://via.placeholder.com/900x250?text=:("} src={url} width="100%"></Image>,
    },
    { title: "Map", dataIndex: "mapName", key: "mapName", width: "30%" },
    { title: "SR", dataIndex: "sr", key: "sr", width: "5%" },
    { title: "BPM", dataIndex: "bpm", key: "bpm", width: "5%" },
    { title: "Length", dataIndex: "length", key: "length", width: "5%" },
    { title: "CS", dataIndex: "cs", key: "cs", width: "5%" },
    { title: "AR", dataIndex: "ar", key: "ar", width: "5%" },
    { title: "OD", dataIndex: "od", key: "od", width: "5%" },
    {
      title: "ID",
      dataIndex: "id",
      key: "id",
      width: "10%",
      render: (id: string) => (
        <a href={`https://osu.ppy.sh/b/${id}`} target="_blank">
          {id}
        </a>
      ),
    },
  ]

  useEffect(() => {
    const getMappool = async () => {
      await fetch(`/api/get_pool/?pool_name=${poolName}`)
        .then((response) => response.json())
        .then((response) => setMappool(response.maps))
    }
    getMappool()
  }, [])

  return (
    <div>
      <Table
        columns={columns}
        dataSource={convertToDataSource(mappool)}
        pagination={false}
        scroll={{ y: windowSize.height - 240 }}
      />
    </div>
  )
}

export default ViewMappool
