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

import CompanySlice from '../../slices/CompanySlice'

const Tables = () => {
  const [messageDanger, setMessageDanger] = useState(null)
  const list = useSelector(state => state.companies.list);
  const dispatch = useDispatch()

  useEffect(() => {
    dispatch(CompanySlice.actions.index());
  },[])

  const handleDelete = (id) => {
    dispatch(CompanySlice.actions.destroy(id))
      .then(() => dispatch(CompanySlice.actions.index()))
      .then(() => setMessageDanger(`Company ${id} Deletado!`));
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
                    <CTableHeaderCell scope="col">Company Type</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Cnpj</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Actions</CTableHeaderCell>
                  </CTableRow>
                </CTableHead>
                <CTableBody>
                  {list.map((elem, idx) => {
                    return (
                      <CTableRow key={idx} >
                        <CTableDataCell>{elem.id}</CTableDataCell>
                        <CTableDataCell>{elem.name}</CTableDataCell>
                        <CTableDataCell>{elem.company_type}</CTableDataCell>
                        <CTableDataCell>{elem.cnpj}</CTableDataCell>
                        <CTableDataCell>
                          <CButtonGroup role="group" aria-label="Row Actions">
                              <CNavLink to={`/companies/${elem.id}`} as={NavLink}>
                                <CButton color="info" variant="outline">Show</CButton>
                              </CNavLink>
                              <CNavLink to={`/companies/${elem.id}/edit`} as={NavLink}>
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
