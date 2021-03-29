import React, { useState, Suspense } from "react"
import { Table, Form, Input, Button, Image, Select } from "antd"
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

  return (
    <Router>
      <Layout style={{ height: "100vh" }}>
        <Header>
          <div
            style={{
              float: "left",
              width: "50px",
              height: "31px",
              margin: "16px 24px 16px 0",
              // background: "rgba(255, 255, 255, 0.3)",
            }}
          >
            <Image
              src="https://cdn.discordapp.com/emojis/810744842970726401.png"
              width="80%"
              preview={false}
            ></Image>
          </div>
          <Menu theme="dark" mode="horizontal">
            <Menu.Item key="2">
              <Button type="link" href="/" style={{ marginLeft: "auto" }}>
                Home
              </Button>
            </Menu.Item>
            <Menu.Item key="3">
              <Button type="link" href="/create" style={{ marginLeft: "auto" }}>
                Create Mappool
              </Button>
            </Menu.Item>
            <Menu.Item key="1" style={{ float: "right" }}>
              <Button type="link" href="/api/login">
                Login
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
