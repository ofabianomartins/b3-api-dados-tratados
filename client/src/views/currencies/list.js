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

import CurrencySlice from '../../slices/CurrencySlice'

const Tables = () => {
  const [messageDanger, setMessageDanger] = useState(null)
  const list = useSelector(state => state.currencies.list);
  const dispatch = useDispatch()

  useEffect(() => {
    dispatch(CurrencySlice.actions.index());
  },[])

  const handleDelete = (id) => {
    dispatch(CurrencySlice.actions.destroy(id))
      .then(() => dispatch(CurrencySlice.actions.index()))
      .then(() => setMessageDanger(`Currency ${id} Deletado!`));
  }

  return (
    <CRow>
      <CCol xs={12}>
        <CCard className="mb-4">
          <CCardBody>
            <CNavLink to='/currencies/create' as={NavLink}>
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
                    <CTableHeaderCell scope="col">name</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Code</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Actions</CTableHeaderCell>
                  </CTableRow>
                </CTableHead>
                <CTableBody>
                  {list.map((elem, idx) => {
                    return (
                      <CTableRow key={idx} >
                        <CTableDataCell>{elem.id}</CTableDataCell>
                        <CTableDataCell>{elem.name}</CTableDataCell>
                        <CTableDataCell>{elem.code}</CTableDataCell>
                        <CTableDataCell>
                          <CButtonGroup role="group" aria-label="Row Actions">
                              <CNavLink to={`/currencies/${elem.id}`} as={NavLink}>
                                <CButton color="info" variant="outline">Show</CButton>
                              </CNavLink>
                              <CNavLink to={`/currencies/${elem.id}/edit`} as={NavLink}>
                                <CButton color="primary" variant="outline">Edit</CButton>
                              </CNavLink>
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
