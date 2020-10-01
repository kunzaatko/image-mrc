#[derive(Debug)]
// TODO: make the structure of the header more acceptable by grouping, like [nx,ny,nz] as dimensions and so on <01-10-20, kunzaatko> //
pub struct Header {
    /// Number of columns in 3D data array
    ///
    /// NOTE: fast axis
    /// NOTE: The data block of an MRC format file holds a 3D array of data (of type specified by `mode`). `nx`, `ny`, `nz` specify the dimensions (in grid points) of this array. In EM, this will correspond to the dimensions of a volume/map, or the combined size of an image/volume stack. In crystallography, this will correspond to the dimensions of a map, which may cover a crystallographic unit cell or may cover some fraction or multiple of a unit cell.
    nx: i32, // 1-4

    /// Number of rows in 3D data array
    /// NOTE: medium axis
    ny: i32, // 5-8

    /// Number of sections in 3D data array
    /// NOTE: slow axis
    nz: i32, // 9-12

    /// Data type
    /// TODO(DOCS): input table of possible modes <01-10-20, kunzaatko>
    /// NOTE: In the MRC2014 format, `mode=0` has been clarified as signed, and `mode=6` has been added for 16-bit unsigned integer data.
    mode: Option<i32>, // 13-16

    /// Number of first column in map (Default = 0)
    nxstart: i32, // 17-20

    /// Number of first row in map (Default = 0)
    nystart: i32, // 21-24

    /// Number of first section in map (Default = 0)
    nzstart: i32, // 25-28

    /// Number of intervals along X of the "unit cell"
    mx: Option<i32>, // 29-32

    /// Number of intervals along Y of the "unit cell"
    my: Option<i32>, // 33-36

    /// Number of intervals along Z of the "unit cell"
    /// NOTE: In crystallographic usage, `mz` represents the number of intervals, or sampling grid, along Z in a crystallographic unit cell. This need not be the same as `nz`, if the map doesn't cover exactly a single unit cell. For microscopy, where there is no unit cell, `mz` represents the number of sections in a single volume. For a volume stack, `nz`/`mz` will be the number of volumes in the stack. For images, `mz` = 1.
    mz: Option<i32>, // 37-40

    /// Cell X length in angstroms
    xlen: Option<f32>, // 41-44

    /// Cell Y length in angstroms
    ylen: Option<f32>, // 45-48

    /// Cell Z length in angstroms
    zlen: Option<f32>, // 41-52

    /// Cell angles in degrees // TODO(DOCS): specify the concrete angles that are defined by these values <01-10-20, kunzaatko> //
    alpha: Option<f32>, // 53-56
    beta: Option<f32>, // 57-60
    gama: Option<f32>, // 61-64

    /// Axis corresponding to columns (1=X, 2=Y, 3=Z)
    /// NOTE: In EM `mapc`,`mapr`,`maps` = 1,2,3 so that sections and images are perpendicular to the Z axis. In crystallography, other orderings are possible. For example, in some spacegroups it is convenient to section along the Y axis (i.e. where this is the polar axis).
    mapc: Option<i32>, // 65-68

    /// Axis corresponding to rows (1=X, 2=Y, 3=Z)
    mapr: Option<i32>, // 69-72

    /// Axis corresponding to sections (1=X, 2=Y, 3=Z)
    maps: Option<i32>, // 73-76

    /// Minimum pixel/density value
    /// NOTE: Density statistics may not be kept up-to-date for image/volume stacks, since it is expensive to recalculate these every time a new image/volume is added/deleted. We have proposed the following convention: `amax` < `amin`, `amean` < min({`amin`, `amax`}), `rms` < 0 each indicate that the quantity in question is not well determined.
    amin: Option<f32>, // 77-80

    /// Maximum pixel/density value
    amax: Option<f32>, // 81-84

    /// Mean pixel/density value
    amean: Option<f32>, // 85-88

    /// Space group number 0 or 1
    /// NOTE: Spacegroup 0 implies a 2D image or image stack. For crystallography, ISPG represents the actual spacegroup. For single volumes from EM/ET, the spacegroup should be 1. For volume stacks, we adopt the convention that `ispg` is the spacegroup number + 400, which in EM/ET will typically be 401.
    ispg: Option<i32>, // 89-92

    /// Number of bytes used for symmetry data (0 or 80)
    ///
    /// NOTE: `nsymbt` specifies the size of the extended header in bytes, whether it contains symmetry records (as in the original format definition) or any other kind of additional metadata.
    nsymbt: Option<i32>, // 93-96

    /// Extra space used for anything
    extra: Option<Extra>, // 97-196

    /// Origin in X,Y,Z used for transforms
    /// NOTE: For transforms (`mode` 3 or 4), `origin` is the phase origin of the transformed image in pixels, e.g. as used in helical processing of the MRC package. For a transform of a padded image, this value corresponds to the pixel position in the padded image of the center of the unpadded image.
    /// NOTE: For other modes, `origin` specifies the real space location of a subvolume taken from a larger volume. In the (2-dimensional) example shown above, the header of the map containing the subvolume (red rectangle) would contain `origin` = 100, 120 to specify its position with respect to the original volume (assuming the original volume has its own `origin` set to 0, 0).
    origin: Option<Origin>, // 197-208

    /// Character string 'MAP ' to identify file type
    map: String, // 209-212

    /// Machine stamp
    /// NOTE: Bytes 213 and 214 contain 4 `nibbles' (half-bytes) indicating the representation of float, complex, integer and character datatypes. Bytes 215 and 216 are unused. The CCP4 library contains a general representation of datatypes, but in practice it is safe to use 0x44 0x44 0x00 0x00 for little endian machines, and 0x11 0x11 0x00 0x00 for big endian machines. The CCP4 library uses this information to automatically byte-swap data if appropriate, when tranferring data files between machines.
    mach_st: String, // 213-216

    /// rms deviation of map from mean density
    rms: Option<f32>, // 217-220

    /// Number of labels being used (lables are 10, 80 character(ASCII) texts included after the header in memory)
    nlabl: i32, //? 221-224

    /// 10 × 80 character text labels
    label: Option<Vec<String>>,
}

#[derive(Debug)]
struct Extra {
    /// Code for the type of extended header
    ///
    /// NOTE: A code for the kind of metadata held in the extended header. Currently agreed values are:
    /// __CCP4__	Format from CCP4 suite
    /// __MRCO__	MRC format
    /// __SERI__	SerialEM. Details in the IMOD documentation.
    /// __AGAR__	Agard
    /// __FEI1__	FEI software, e.g. EPU and Xplore3D, Amira, Avizo. Documented in the EPU User Manual, Appendix C.
    /// __HDF5__	Metadata in HDF5 format
    ext_type: String, // 105-108

    /// Version of the MRC format
    /// NOTE: The version of the MRC format that the file adheres to, specified as a 32-bit integer and calculated as:
    /// - Year * 10 + version within the year (base 0)
    /// NOTE: For the current format change, the value would be 20140.
    nversion: i32,
}

#[derive(Debug)]
struct Origin {
    xorg: f32,
    yorg: f32,
    zorg: f32,
}
