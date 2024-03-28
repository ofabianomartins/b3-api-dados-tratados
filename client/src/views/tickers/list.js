
import React, {useEffect, useState} from 'react'
import { NavLink } from 'react-router-dom'
import {
  CButton,
  CButtonGroup,
  CAlert,
  CCard,
  CCardBody,
  CNavLink,
  CCol,
  CRow,
  CTable,
  CTableBody,
  CTableDataCell,
  CTableHead,
  CTableHeaderCell,
  CTableRow,
} from '@coreui/react'

import { useDispatch, useSelector } from 'react-redux'

import TickerSlice from '../../slices/TickerSlice'

const Tables = () => {
  const [messageDanger, setMessageDanger] = useState(null)
  const list = useSelector(state => state.tickers.list);
  const dispatch = useDispatch()

  useEffect(() => {
    dispatch(TickerSlice.actions.index());
  },[])

  const handleDelete = (id) => {
    dispatch(TickerSlice.actions.destroy(id))
      .then(() => dispatch(TickerSlice.actions.index()))
      .then(() => setMessageDanger(`Ticker ${id} Deletado!`));
  }

  return (
    <CRow>
      <CCol xs={12}>
        <CCard className="mb-4">
          <CCardBody>
            <CNavLink to='/tickers/create' as={NavLink}>
              <CButton color="info" variant="outline">Create</CButton>
            </CNavLink>
          </CCardBody>
        </CCard>
        {messageDanger && <CAlert color="danger"> {messageDanger} </CAlert>}
        <CCard className="mb-4">
          <CCardBody>
              <CTable>
                <CTableHead>
                  <CTableRow>
                    <CTableHeaderCell scope="col">#</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Symbol</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Security Type</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Actions</CTableHeaderCell>
                  </CTableRow>
                </CTableHead>
                <CTableBody>
                  {list.map((elem, idx) => {
                    return (
                      <CTableRow key={idx} >
                        <CTableDataCell>{elem.id}</CTableDataCell>
                        <CTableDataCell>{elem.symbol}</CTableDataCell>
                        <CTableDataCell>{elem.security_type}</CTableDataCell>
                        <CTableDataCell>
                          <CButtonGroup role="group" aria-label="Row Actions">
                              <CButton color="info" variant="outline">Show</CButton>
                              <CButton color="primary" variant="outline">Edit</CButton>
                              <CButton
                                color="danger"
                                variant="outline"
                                onClick={() => handleDelete(elem.id)}
                              >
                                Delete
                              </CButton>
                          </CButtonGroup>
                        </CTableDataCell>
                      </CTableRow>
                    )
                  })}
                </CTableBody>
              </CTable>
          </CCardBody>
        </CCard>
      </CCol>
    </CRow>
  )
}

export default Tables
