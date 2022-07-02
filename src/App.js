import React, { useEffect, useState } from 'react';
import './App.css';
import { Button, Layout, Menu, Input, Space, Table, notification, Select, Col, Row } from 'antd';
import { OmitProps } from 'antd/lib/transfer/ListBody';
import { DownloadOutlined } from '@ant-design/icons';
import { open } from '@tauri-apps/api/dialog';
import { appDir } from '@tauri-apps/api/path';
const { Header, Content, Footer } = Layout;
const { Search } = Input;
const { Option } = Select;

const invoke = window.__TAURI__.invoke

const openNotification = (title, message, type) => {
  notification[type]({
    message: title,
    description: message
  });
};

const App = () => {
  const [data, setData] = useState([]);
  const [pageNumber, setPageNumber] = useState(1);
  const [quality, setQuality] = useState('SQ');
  const [keyWord, setKeyWord] = useState('');
  const [total, setTotal] = useState(0);
  const [loadings, setLoadings] = useState([]);
  const pageSize = 20;

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
      key: 'downloadUrl',
      dataIndex: 'downloadUrl',
      render: (_, { title, downloadUrl }, index) => (
        <Space size="middle">
          <Button onClick={() => download(title, downloadUrl, index)} loading={loadings[index]} type="primary" shape="circle" icon={<DownloadOutlined />} size='small' />
        </Space>
      ),
    },
  ];


  const search = () => {
    invoke('search', {
      keyWord: keyWord,
      pageNumber: pageNumber,
      pageSize: pageSize,
      quality: quality,
    }).then(res => {
      console.log("invoke return: ", res)
      // data = res.data
      const newData = res.songs.map(item => {
        return {
          key: item.id,
          title: item.name,
          artist: item.singers.map(singer => singer.name).join(', '),
          album: item.albums.map(album => album.name).join(', '),
          downloadUrl: item.download_url
        }
      })
      setTotal(res.total)
      console.log(total)
      setData(newData)
      setLoadings([])
    });
  }

  const setLoading = (index, loading) => {
    setLoadings((prevLoadings) => {
      const newLoadings = [...prevLoadings];
      newLoadings[index] = loading;
      return newLoadings;
    });
  }

  const download = (title, url, index) => {
    console.log('download: ', title, url)
    setLoading(index, true)
    appDir().then(dir => {
      open({
        directory: true,
        multiple: false,
        defaultPath: dir,
      }).then(selectedPath => {
        if (!selectedPath) {
          setLoading(index, false)
          return;
        }
        console.log('selectedPath: ', selectedPath)
        openNotification("Downloading", "Downloading " + title, "info")
        invoke('download', { name: title, url: url, path: selectedPath }).then(() => {
          openNotification('Download Success', `${title} download success`, "success")
          setLoading(index, false)
        })
      })
    })
  };

  const onSearch = (value) => {
    console.log('search: ', value)
    setKeyWord(value)
  };

  const onPageNumberChanged = (value) => {
    console.log('pageNumber: ', value)
    setPageNumber(value)
  };

  useEffect(() => {
    search()
  }, [pageNumber, keyWord, quality])

  return (
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
        <Row style={{ marginTop: '20px', marginBottom: '20px' }}>
          <Col span={4} >
              <Select
                labelInValue
                defaultValue={{
                  value: 'SQ',
                  label: '无损',
                }}
                style={{ width: '80%' }}
                onChange={(opt) => {
                  console.log('qualit: ', opt.value)
                  setQuality(opt.value)
                }}
              >
                <Option value="SQ">无损</Option>
                <Option value="HQ">高品质</Option>
              </Select>
          </Col>
          <Col span={20}>
            <Search
              placeholder="input search text"
              allowClear
              enterButton="Search"
              size="middle"
              onSearch={onSearch}
            />
          </Col>
        </Row>
        <Table
          columns={columns}
          dataSource={data}
          size='small'
          pagination={{
            onChange: onPageNumberChanged,
            defaultPageSize: pageSize,
            showTotal: (total, range) => `${range[0]}-${range[1]} of ${total} items`,
            total: total,
            showSizeChanger: false
          }} />
      </Content>
      <Footer
        style={{
          textAlign: 'center',
        }}
      >
        Migu Music DL ©2022 Created by <a href='https://github.com/swim2sun/migu-music-dl'>swim2sun</a>
      </Footer>
    </Layout>
  )
}

export default App;