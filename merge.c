#include <Python.h>
#include <stdio.h>

#include "merge.h"

/* Heterogeneous compare: default, always safe to fall back on. */
static inline int object_compare(PyObject *v, PyObject *w)
{
    /* No assumptions necessary! */
    return PyObject_RichCompareBool(v, w, Py_LT);
}

/* Float compare: compare any two floats. */
static inline int float_compare( PyObject* v, PyObject* w )
{
	int res;

	/* Modified from Objects/floatobject.c:float_richcompare, assuming: */
	assert( Py_IS_TYPE( v, &PyFloat_Type ) );
	assert( Py_IS_TYPE( w, &PyFloat_Type ) );

	res = PyFloat_AS_DOUBLE( v ) < PyFloat_AS_DOUBLE( w );
	assert( res == PyObject_RichCompareBool( v, w, Py_LT ) );
	return res;
}

/* Bounded int compare: compare any two longs that fit in a single machine word. */
static inline int long_compare( PyObject* v, PyObject* w )
{
	PyLongObject *vl, *wl;
	sdigit v0, w0;
	int res;

	/* Modified from Objects/longobject.c:long_compare, assuming: */
	assert( Py_IS_TYPE( v, &PyLong_Type ) );
	assert( Py_IS_TYPE( w, &PyLong_Type ) );
	assert( Py_ABS( Py_SIZE( v ) ) <= 1 );
	assert( Py_ABS( Py_SIZE( w ) ) <= 1 );

	vl = (PyLongObject*)v;
	wl = (PyLongObject*)w;

	v0 = Py_SIZE( vl ) == 0 ? 0 : (sdigit)vl->ob_digit[0];
	w0 = Py_SIZE( wl ) == 0 ? 0 : (sdigit)wl->ob_digit[0];

	if( Py_SIZE( vl ) < 0 )
		v0 = -v0;
	if( Py_SIZE( wl ) < 0 )
		w0 = -w0;

	res = v0 < w0;
	assert( res == PyObject_RichCompareBool( v, w, Py_LT ) );
	return res;
}
/* Latin string compare: safe for any two latin (one byte per char) strings. */
static inline int latin_compare( PyObject* v, PyObject* w )
{
	Py_ssize_t len;
	int res;

	/* Modified from Objects/unicodeobject.c:unicode_compare, assuming: */
	assert( Py_IS_TYPE( v, &PyUnicode_Type ) );
	assert( Py_IS_TYPE( w, &PyUnicode_Type ) );
	assert( PyUnicode_KIND( v ) == PyUnicode_KIND( w ) );
	assert( PyUnicode_KIND( v ) == PyUnicode_1BYTE_KIND );

	len = Py_MIN( PyUnicode_GET_LENGTH( v ), PyUnicode_GET_LENGTH( w ) );
	res = memcmp( PyUnicode_DATA( v ), PyUnicode_DATA( w ), len );

	res = ( res != 0 ? res < 0 : PyUnicode_GET_LENGTH( v ) < PyUnicode_GET_LENGTH( w ) );

	assert( res == PyObject_RichCompareBool( v, w, Py_LT ) );
	;
	return res;
}

static inline PyObject*
merge_internal( PyObject* self, PyObject* args, int ( *compare_ptr )( PyObject*, PyObject* ) )
{
	int i;
	int n1, n2;
	int i1 = 0, i2 = 0;
	int result;

	PyObject* listObj1 = NULL;
	PyObject* listObj2 = NULL;
	PyObject* elem1 = NULL;
	PyObject* elem2 = NULL;

	if( !PyArg_ParseTuple( args, "O!O!", &PyList_Type, &listObj1, &PyList_Type, &listObj2 ) )
		return NULL;

	n1 = PyList_Size( listObj1 );
	n2 = PyList_Size( listObj2 );

	/* TODO check where reference counting is required */
	PyObject* mergedList = PyList_New( n1 + n2 );

	/* should raise an error here. */
	if( n1 < 0 || n2 < 0 )
		return NULL; /* Not a list */

	for( i = 0;; ) {
		if( i1 < n1 ) {
			if( !elem1 ) {
				elem1 = PyList_GetItem( listObj1, i1 );
				Py_INCREF( elem1 );
			}
			if( i2 < n2 ) {
				if( !elem2 ) {
					elem2 = PyList_GetItem( listObj2, i2 );
					Py_INCREF( elem2 );
				}
				result = compare_ptr( elem1, elem2 );
				switch( result ) {
				case 1:
					PyList_SetItem( mergedList, i++, elem1 );
					elem1 = NULL;
					i1++;
					break;

				case 0:
					PyList_SetItem( mergedList, i++, elem2 );
					elem2 = NULL;
					i2++;
					break;

				default:
					// error occured
					printf( "ERROR!\n" );
					// TODO release references
					return NULL; /* TODO should raise an error */
				}
			}
			else {
				PyList_SetItem( mergedList, i++, elem1 );
				elem1 = NULL;
				i1++;
			}
		}
		else {
			if( i2 < n2 ) {
				elem2 = PyList_GetItem( listObj2, i2 );
				Py_INCREF( elem2 );
				PyList_SetItem( mergedList, i++, elem2 );
				elem2 = NULL;
				i2++;
			}
			else {
				break;
			}
		}
	}

	return mergedList;
}

PyObject* merge( PyObject* self, PyObject* args )
{
	return merge_internal( self, args, object_compare );
}

PyObject* merge_latin( PyObject* self, PyObject* args )
{
	return merge_internal( self, args, latin_compare );
}

PyObject* merge_int( PyObject* self, PyObject* args )
{
	return merge_internal( self, args, long_compare );
}

PyObject* merge_float( PyObject* self, PyObject* args )
{
	return merge_internal( self, args, float_compare );
}