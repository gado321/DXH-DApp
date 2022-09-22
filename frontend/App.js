import 'regenerator-runtime/runtime';
import React from 'react';
import { Table } from 'antd';
import { AndroidOutlined, AppleOutlined } from '@ant-design/icons';
import { Tabs } from 'antd';
import { Layout } from 'antd';
const { Header, Footer, Sider, Content } = Layout;

import './assets/global.css';

import { EducationalText, GetCandidate, SignInPrompt, SignOutButton } from './ui-components';

const columns = [
  {
    title: 'Wallet address',
    dataIndex: 'walletAddress',
    width: '40%',
  },
  {
    title: 'Age',
    dataIndex: 'age',
    width: '10%',
  },
  {
    title: 'Description',
    dataIndex: 'description',
  },
];
const data = [];

for (let i = 0; i < 100; i++) {
  data.push({
    key: i,
    walletAddress: `Edward King ${i}`,
    age: 32,
    description: `London, Park Lane no. ${i}`,
  });
}

export default function App(props) {
  /// If user not signed-in with wallet - show prompt
  if (!props.isSignedIn) {
    // Sign-in flow will reload the page later
    return <SignInPrompt wallet={props.wallet}/>;
  }

  return (
    <>
      <main>
        <div className="content">
          <div className="header">
            <SignOutButton wallet={props.wallet}/>
          </div>
          <div className="candidate-table-place">
            <div className='raw'>
              <Table style={{}}
                columns={columns}
                dataSource={data}
                pagination={{
                  pageSize: 50,
                }}
                scroll={{
                  y: 240,
                }}
              />
            </div>

            <div className='verified'>
              <Table style={{}}
                columns={columns}
                dataSource={data}
                pagination={{
                  pageSize: 50,
                }}
                scroll={{
                  y: 240,
                }}
              />
            </div>
          </div>
          <div className="submit-place">
            <div className="donate-form">   
              <form>
                <label>
                  Name:
                  <input type="text" name="Number of Token" placeholder='Number of token'/>
                </label>
                <input type="submit" value="Donate"/>
              </form>
            </div>

            <div className="submit-form">   
              <form>
                <label>
                  Name:
                  <input type="text" name="Cadidate id" placeholder='Candidate id'/>
                </label>
                <input type="submit" value="Submit" />
              </form>
            </div>      
          </div>
        </div>
        
      </main>
    </>
  );
}
