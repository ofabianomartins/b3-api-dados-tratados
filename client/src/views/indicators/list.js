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

import IndicatorSlice from '../../slices/IndicatorSlice'

const Tables = () => {
  const [messageDanger, setMessageDanger] = useState(null)
  const list = useSelector(state => state.indicators.list);
  const dispatch = useDispatch()

  useEffect(() => {
    dispatch(IndicatorSlice.actions.index());
  },[])

  const handleDelete = (id) => {
    dispatch(IndicatorSlice.actions.destroy(id))
      .then(() => dispatch(IndicatorSlice.actions.index()))
      .then(() => setMessageDanger(`Indicator ${id} Deletado!`));
  }

  return (
    <CRow>
      <CCol xs={12}>
        <CCard className="mb-4">
          <CCardBody>
            <CNavLink to='/indicators/create' as={NavLink}>
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
                    <CTableHeaderCell scope="col">Symbol</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Description</CTableHeaderCell>
                    <CTableHeaderCell scope="col">IndicatorType</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Actions</CTableHeaderCell>
                  </CTableRow>
                </CTableHead>
                <CTableBody>
                  {list.map((elem, idx) => {
                    return (
                      <CTableRow key={idx} >
                        <CTableDataCell>{elem.id}</CTableDataCell>
                        <CTableDataCell>{elem.name}</CTableDataCell>
                        <CTableDataCell>{elem.symbol}</CTableDataCell>
                        <CTableDataCell>{elem.description}</CTableDataCell>
                        <CTableDataCell>{elem.indicator_type}</CTableDataCell>
                        <CTableDataCell>
                          <CButtonGroup role="group" aria-label="Row Actions">
                              <CNavLink to={`/indicators/${elem.id}`} as={NavLink}>
                                <CButton color="info" variant="outline">Show</CButton>
                              </CNavLink>
                              <CNavLink to={`/indicators/${elem.id}/edit`} as={NavLink}>
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
