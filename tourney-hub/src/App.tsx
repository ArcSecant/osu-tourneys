import React, { useState, Suspense } from "react"
import { Table, Form, Input, Button, Select } from "antd"
import { Layout, Menu, Breadcrumb } from "antd"
import { Divider } from "antd"
import { FormInstance } from "antd/lib/form"
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom"

import { MappoolMaker } from "./MappoolMaker"
import ViewMappool from "./Mappool"

const { Header, Content, Footer } = Layout

function App() {
  let formRef = React.createRef<FormInstance>()
  let [poolLink, setPoolLink] = useState<string>("")

  let getLink = () => {
    return formRef.current!.getFieldValue("poolName")
  }

  return (
    <Router>
      <Layout style={{ height: "100vh" }}>
        <Header>
          <div
            style={{
              float: "left",
              width: "120px",
              height: "31px",
              margin: "16px 24px 16px 0",
              background: "rgba(255, 255, 255, 0.3)",
            }}
          />
          <Menu theme="dark" mode="horizontal">
            <Menu.Item key="1">
              <Button type="link" href="/create" style={{ marginLeft: "auto" }}>
                Create Mappool
              </Button>
            </Menu.Item>
          </Menu>
        </Header>
        <Content style={{ padding: "25px 35px" }}>
          <Form layout={"inline"} ref={formRef} color="inherit">
            <Form.Item name="poolName" label="Go to" initialValue={""}>
              <Input
                placeholder="Pool Name"
                value={poolLink}
                onChange={(e) => setPoolLink(e.target.value)}
                allowClear
              />
            </Form.Item>
            <Button type="link" href={`/mappools/${poolLink}`}>
              Go
            </Button>
          </Form>
          <Divider />
          <Switch>
            <Route path="/create" children={<MappoolMaker />} />
            <Route path="/mappools/:poolName" children={<ViewMappool />} />
          </Switch>
        </Content>
        <Footer style={{ textAlign: "center" }}>OwO</Footer>
      </Layout>
    </Router>
  )
}

export default App
