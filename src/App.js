import React from 'react';
import './App.css';
import { Breadcrumb, Layout, Menu, Input, Space, Table, Tag } from 'antd';
const { Header, Content, Footer } = Layout;
const { Search } = Input;
const columns = [
  {
    title: 'Title',
    dataIndex: 'title',
    key: 'title',
    render: (text) => <strong>{text}</strong>,
  },
  {
    title: 'Artist',
    dataIndex: 'artist',
    key: 'artist',
  },
  {
    title: 'Album',
    dataIndex: 'album',
    key: 'album',
  },
  {
    title: 'Action',
    key: 'action',
    render: (_, record) => (
      <Space size="middle">
        <a>Download</a>
      </Space>
    ),
  },
];
const data = [
  {
    key: '1',
    title: 'John Brown',
    artist: 'New York No. 1 Lake Park',
    album: 'test'
  },
  {
    key: '2',
    title: 'Jim Green',
    artist: 'London No. 1 Lake Park',
    album: 'test'
  },
  {
    key: '3',
    title: 'Joe Black',
    artist: 'Sidney No. 1 Lake Park',
    album: 'test'
  },
];



const App = () => (
  <Layout className="layout">
    <Header>
      <div className="logo" />
      <Menu
        theme="dark"
        mode="horizontal"
        defaultSelectedKeys={['2']}
        items={[{ key: '1', label: 'Migu Music Download' }]}
      />
    </Header>
    <Content
      style={{
        padding: '0 50px',
      }}
    >
      <Search
        placeholder="input search text"
        allowClear
        enterButton="Search"
        size="large"
        style={{ marginTop: '20px', marginBottom: '20px'}}
        // onSearch={onSearch}
      />
      <Table columns={columns} dataSource={data} />
    </Content>
    <Footer
      style={{
        textAlign: 'center',
      }}
    >
      Ant Design ©2018 Created by Ant UED
    </Footer>
  </Layout>
);

export default App;