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

import SubsectorSlice from '../../slices/SubsectorSlice'

const Tables = () => {
  const [messageDanger, setMessageDanger] = useState(null)
  const list = useSelector(state => state.subsectors.list);
  const dispatch = useDispatch()

  useEffect(() => {
    dispatch(SubsectorSlice.actions.index());
  },[])

  const handleDelete = (id) => {
    dispatch(SubsectorSlice.actions.destroy(id))
      .then(() => dispatch(SubsectorSlice.actions.index()))
      .then(() => setMessageDanger(`Subsector ${id} Deletado!`));
  }

  return (
    <CRow>
      <CCol xs={12}>
        <CCard className="mb-4">
          <CCardBody>
            <CNavLink to='/subsectors/create' as={NavLink}>
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
                    <CTableHeaderCell scope="col">Sector</CTableHeaderCell>
                    <CTableHeaderCell scope="col">Actions</CTableHeaderCell>
                  </CTableRow>
                </CTableHead>
                <CTableBody>
                  {list.map((elem, idx) => {
                    return (
                      <CTableRow key={idx} >
                        <CTableDataCell>{elem.id}</CTableDataCell>
                        <CTableDataCell>{elem.name}</CTableDataCell>
                        <CTableDataCell>
                          <CNavLink to={`/sectors/${elem.sector_id}`} as={NavLink}>
                            #{elem.sector_id}
                          </CNavLink>
                        </CTableDataCell>
                        <CTableDataCell>
                          <CButtonGroup role="group" aria-label="Row Actions">
                              <CNavLink to={`/subsectors/${elem.id}`} as={NavLink}>
                                <CButton color="info" variant="outline">Show</CButton>
                              </CNavLink>
                              <CNavLink to={`/subsectors/${elem.id}/edit`} as={NavLink}>
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
